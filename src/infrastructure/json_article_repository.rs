use crate::model::{Article, ArticleRepository};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct JsonArticle {
    title: String,
}

impl JsonArticle {
    pub fn from_article(article: &Article) -> JsonArticle {
        JsonArticle {
            title: String::from(&article.title)
        }
    }
}

pub struct JsonArticleRepository {
    root_path: PathBuf,
}

impl JsonArticleRepository {
    pub fn from_path(path: &Path) -> JsonArticleRepository {
        JsonArticleRepository {
            root_path: path.to_path_buf(),
        }
    }

    pub fn save(&self, article: &Article) -> io::Result<()> {
        let mut article_path = self.root_path.clone();
        article_path.push(article.get_path());
        article_path.set_extension("json");

        println!("Create {}", article_path.to_string_lossy());
        
        fs::create_dir_all(article_path.parent().unwrap())?;

        let json_article = JsonArticle::from_article(article);
        let json = serde_json::to_string(&json_article).unwrap();

        fs::write(article_path, json)?;

        Ok(())
    }
}

impl ArticleRepository for JsonArticleRepository {
    fn get_all(&self) -> Vec<Article> {
        panic!("ArticleRepository::get_all not implemented");
    }
}
