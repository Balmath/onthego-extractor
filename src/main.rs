mod document;
mod model;

use mongodb::Client;
use crate::document::Article as ArticleDocument;
use crate::model::Article;

fn main() {
    let dbs_uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(dbs_uri).expect("Cannot connect to MongoDB server");

    println!("List of database:");

    for db_name in client.list_database_names(None).unwrap() {
        println!("{}", db_name);
    }

    let db_name = "rapog_sport_release";
    let db = client.database(db_name);

    println!("List of collection in {}:", db_name);

    for collection_name in db.list_collection_names(None).unwrap() {
        println!("{}", collection_name);
    }

    println!("List of article:");

    let collection = db.collection("article");
    for document_result in collection.find(None, None).unwrap() {
        let document = document_result.unwrap();

        let article_document = ArticleDocument::from_document(document).unwrap();
        let article = Article::from(article_document);

        println!("{} - {} - {}", article.language, article.category, article.title);
    }
}

