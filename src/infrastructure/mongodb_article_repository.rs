use bson::{from_bson, Document, Bson, DecoderError, UtcDateTime};
use chrono::{DateTime, Utc};
use crate::model::{Article, ArticleRepository};
use mongodb::{Client, Collection};
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
struct BsonArticle {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub language: String,
    pub category: String,
    pub title: String,
    pub author: String,
    pub date: UtcDateTime,
    pub sub_category: Option<String>,
    pub summary_image: String,
    pub summary_text: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl TryFrom<Document> for BsonArticle {
    type Error = DecoderError;

    fn try_from(document: Document) -> Result<Self, Self::Error> {
        from_bson(Bson::Document(document))
    }
}

impl From<BsonArticle> for Article {
    fn from(bson_article: BsonArticle) -> Self {
        let article_content = format!(
            "<img src=\"/static/images/{}\">\n<p>\n<strong>{}</strong>\n</p>\n<br>\n{}",
            bson_article.summary_image,
            bson_article.summary_text,
            bson_article.content);

        Article {
            language: bson_article.language,
            category: bson_article.category,
            sub_category: bson_article.sub_category,
            title: bson_article.title,
            author: bson_article.author,
            date: DateTime::<Utc>::from(bson_article.date),
            content: article_content,
            tags: bson_article.tags,
        }
    }
}

pub struct MongodbArticleRepository {
    uri: String
}

impl MongodbArticleRepository {
    pub fn with_uri_str(uri: &str) -> Self {
        Self { uri: String::from(uri) }
    }

    fn get_article_collection(&self) -> Collection {
        let client = Client::with_uri_str(&self.uri).expect("Cannot connect to MongoDB server");
        let db_name = "rapog_sport_release";
        let db = client.database(db_name);
        db.collection("article")
    }
}

impl ArticleRepository for MongodbArticleRepository {
    fn get_all(&self) -> Vec<Article> {
        let mut articles: Vec<Article> = vec![];
        let collection = self.get_article_collection();

        for document_result in collection.find(None, None).unwrap() {
            let document = document_result.unwrap();

            let bson_article = BsonArticle::try_from(document).unwrap();

            articles.push(bson_article.into());
        }

        return articles;
    }
}
