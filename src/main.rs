mod infrastructure;
mod model;

use crate::infrastructure::{JsonArticleRepository, MongodbArticleRepository};
use crate::model::ArticleRepository;
use std::path::Path;

fn main() {
    let uri = "mongodb://localhost:27017";
    let mongodb_repository = MongodbArticleRepository::with_uri_str(uri);
    let root_path = Path::new("~/onthego-extractor/build");
    let json_repository = JsonArticleRepository::from_path(root_path);

    for article in mongodb_repository.get_all() {
        json_repository.save(&article);
    }
}

