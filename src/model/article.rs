use slugify::slugify;
use std::path::PathBuf;

pub struct Article {
    pub language: String,
    pub category: String,
    pub title: String
}

impl Article {
    pub fn get_path(&self) -> String {
        let slug_title = slugify!(&self.title, separator = "_");

        let path: PathBuf = [&self.language, &self.category, &slug_title].iter().collect();

        String::from(path.to_string_lossy())
    }
}
