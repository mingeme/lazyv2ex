#[derive(Debug, Clone, PartialEq)]
pub struct Topic {
    pub title: String,
    pub author: String,
    pub comment: String,
    pub content: String,
    pub updated: String,
    pub link: String,
    pub replies: Vec<Reply>,
}

impl Topic {
    pub fn new(
        title: String,
        author: String,
        comment: String,
        content: String,
        updated: String,
        link: String,
    ) -> Self {
        Self {
            title,
            author,
            comment,
            content,
            updated,
            link,
            replies: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reply {
    pub author: String,
    pub time: String,
    pub content: String,
    pub number: String,
    pub reply_to: String,
    pub reply_count: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopicDetail {
    pub title: String,
    pub content: String,
    pub author: String,
    pub comment: String,
    pub updated: String,
    pub link: String,
    pub replies: Vec<Reply>,
}
