use atom_syndication::Feed;
use color_eyre::Result;

use crate::models::Topic;

const V2EX_RSS_URL: &str = "https://www.v2ex.com/feed/tab/all.xml";

pub fn fetch_topics() -> Result<Vec<Topic>> {
    let content = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .build()?
        .get(V2EX_RSS_URL)
        .send()?
        .bytes()?;

    let feed = Feed::read_from(content.as_ref())?;

    let topics: Vec<Topic> = feed
        .entries()
        .iter()
        .map(|entry| {
            // Extract comment count from content if available
            let comment = entry
                .links()
                .first()
                .map(|c| c.href())
                .and_then(|content| {
                    content
                        .split("#reply")
                        .nth(1)
                        .and_then(|s| s.trim().parse::<String>().ok())
                })
                .unwrap_or_else(|| "0".to_string());

            Topic::new(
                entry.title().to_string(),
                entry
                    .authors()
                    .get(0)
                    .map_or("".to_string(), |a| a.name().to_string()),
                comment,
                entry.updated().to_string(),
            )
        })
        .collect();

    Ok(topics)
}
