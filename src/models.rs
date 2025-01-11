#[derive(Debug, Clone)]
pub struct Topic {
    title: String,
    author: String,
    comment: String,
    updated: String,
}

impl Topic {
    pub fn new(title: String, author: String, comment: String, updated: String) -> Self {
        Self {
            title,
            author,
            comment,
            updated,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn author(&self) -> &str {
        &self.author
    }
    pub fn comment(&self) -> &str {
        &self.comment
    }
    pub fn updated(&self) -> &str {
        &self.updated
    }
}
