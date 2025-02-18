use atom_syndication::Feed;
use chrono::Utc;
use color_eyre::Result;
use scraper::{Html, Selector};

use crate::model::{Reply, Topic};
use crate::time::time_formatting::format_relative_time;

const V2EX_RSS_URL: &str = "https://www.v2ex.com/feed/tab/all.xml";

pub struct Crawler {
    client: reqwest::blocking::Client,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
                .build()
                .unwrap(),
        }
    }

    pub fn fetch_topics(&self) -> Result<Vec<Topic>> {
        let content = self.client.get(V2EX_RSS_URL).send()?.bytes()?;

        let feed = Feed::read_from(content.as_ref())?;

        let topics: Vec<Topic> = feed
            .entries()
            .iter()
            .map(|entry| {
                let link = entry.links().first().map(|c| c.href());

                // Extract comment count from content if available
                let comment = link
                    .and_then(|content| {
                        content
                            .split("#reply")
                            .nth(1)
                            .and_then(|s| s.trim().parse::<String>().ok())
                    })
                    .unwrap_or_else(|| "0".to_string());

                // Format the time
                let updated = format_relative_time(entry.updated().with_timezone(&Utc));

                Topic::new(
                    entry.title().to_string(),
                    entry
                        .authors()
                        .first()
                        .map_or("".to_string(), |a| a.name().to_string()),
                    comment,
                    String::new(),
                    updated,
                    link.unwrap().to_string(),
                )
            })
            .collect();

        Ok(topics)
    }

    pub fn fetch_topic_detail(&self, url: &str) -> Result<Topic> {
        let resp = self.client.get(url).send()?.text()?;
        let document = Html::parse_document(&resp);

        // Selectors
        let title_selector = Selector::parse("h1").unwrap();
        let author_selector = Selector::parse(".header small a").unwrap();
        let time_selector = Selector::parse(".header small span").unwrap();
        let content_selector = Selector::parse(".topic_content").unwrap();
        let replies_selector = Selector::parse(".cell[id^='r_']").unwrap();

        let title = document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let author = document
            .select(&author_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let time = document
            .select(&time_selector)
            .next()
            .and_then(|el| el.value().attr("title"))
            .unwrap_or_default()
            .to_string();

        let content = document
            .select(&content_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();

        let mut topic = Topic::new(
            title,
            author,
            "0".to_string(),
            content,
            time,
            url.to_string(),
        );

        // Parse replies
        let mut replies = Vec::new();
        for element in document.select(&replies_selector) {
            let reply_content_selector = Selector::parse(".reply_content").unwrap();
            let reply_author_selector = Selector::parse("strong a").unwrap();
            let reply_time_selector = Selector::parse(".ago").unwrap();
            let reply_number_selector = Selector::parse(".no").unwrap();

            let content = element
                .select(&reply_content_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let author = element
                .select(&reply_author_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let time = element
                .select(&reply_time_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let number = element
                .select(&reply_number_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let reply_to = if content.trim().starts_with('@') {
                content
                    .split_whitespace()
                    .next()
                    .map(|s| s[1..].to_string())
                    .unwrap_or_default()
            } else {
                String::new()
            };

            replies.push(Reply {
                author,
                time,
                content,
                number,
                reply_to,
                reply_count: 0,
            });
        }

        // Calculate reply counts
        for i in 0..replies.len() {
            let count = replies
                .iter()
                .filter(|r| r.reply_to == replies[i].author)
                .count() as i32;
            replies[i].reply_count = count;
        }

        topic.replies = replies;
        Ok(topic)
    }
}

#[test]
fn test_crawler() {
    let crawler = Crawler::new();
    let topic = crawler
        .fetch_topic_detail("https://www.v2ex.com/t/1111950")
        .unwrap();
    println!("{:#?}", topic);
}
