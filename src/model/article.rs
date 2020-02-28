use chrono::{DateTime, Utc};
use slugify::slugify;
use std::path::PathBuf;

pub struct Article {
    pub language: String,
    pub category: String,
    pub sub_category: Option<String>,
    pub title: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub content: String,
    pub tags: Vec<String>,
}

impl Article {
    pub fn get_path(&self) -> PathBuf {
        let mut path_elements = vec![&self.language, &self.category];

        if let Some(sub_category) = &self.sub_category {
            path_elements.push(&sub_category);
        }

        let slug_title = slugify!(&self.title, separator = "_");
        path_elements.push(&slug_title);
        
        path_elements.iter().collect()
    }
}
