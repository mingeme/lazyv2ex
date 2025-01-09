use color_eyre::Result;
use rss::Channel;
use chrono::DateTime;

const V2EX_RSS_URL: &str = "https://www.v2ex.com/index.xml";

#[derive(Debug, Clone)]
pub struct Topic {
    pub title: String,
    pub author: String,
    pub comments: u32,
    pub pub_date: DateTime<chrono::Utc>,
    pub link: String,
}

pub fn fetch_topics() -> Result<Vec<Topic>> {
    let content = reqwest::blocking::get(V2EX_RSS_URL)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    
    let topics = channel
        .items()
        .iter()
        .filter_map(|item| {
            let title = item.title()?.to_string();
            let author = item.author()?.to_string();
            let link = item.link()?.to_string();
            
            // Parse comments count from description
            let comments = item
                .description()
                .and_then(|desc| {
                    desc.split("评论 ")
                        .nth(1)?
                        .split(' ')
                        .next()?
                        .parse::<u32>()
                        .ok()
                })
                .unwrap_or(0);

            let pub_date = item
                .pub_date()
                .and_then(|date| DateTime::parse_from_rfc2822(date).ok())
                .map(|date| date.with_timezone(&chrono::Utc))
                .unwrap_or_else(chrono::Utc::now);

            Some(Topic {
                title,
                author,
                comments,
                pub_date,
                link,
            })
        })
        .collect();

    Ok(topics)
}
