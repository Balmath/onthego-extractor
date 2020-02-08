use bson::{from_bson, Document, Bson, DecoderError};
use crate::model::{Article, ArticleRepository};
use mongodb::{Client, Collection};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct BsonArticle {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub language: String,
    pub category: String,
    pub title: String
}

impl BsonArticle {
    fn from_document(document: Document) -> Result<BsonArticle, DecoderError> {
        return from_bson(Bson::Document(document));
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

            let bson_article = BsonArticle::from_document(document).unwrap();

            let article = Article {
                language: bson_article.language,
                category: bson_article.category,
                title: bson_article.title,
            };

            articles.push(article);
        }

        return articles;
    }
}
