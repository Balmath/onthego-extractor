use slugify::slugify;
use std::path::PathBuf;

pub struct Article {
    pub language: String,
    pub category: String,
    pub title: String
}

impl Article {
    pub fn get_path(&self) -> PathBuf {
        let slug_title = slugify!(&self.title, separator = "_");

        [&self.language, &self.category, &slug_title].iter().collect()
    }
}
