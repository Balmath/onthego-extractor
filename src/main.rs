mod infrastructure;
mod model;

use crate::infrastructure::MongodbArticleRepository;
use crate::model::ArticleRepository;

fn main() {
    let uri = "mongodb://localhost:27017";
    let repository = MongodbArticleRepository::with_uri_str(uri);

    for article in repository.get_all() {
        println!("{}", article.get_path());
    }
}

