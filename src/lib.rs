mod infrastructure;
mod model;

use crate::infrastructure::{JsonArticleRepository, MongodbArticleRepository};
use crate::model::ArticleRepository;
use std::error::Error;
use std::path::Path;

pub struct Config {
    pub output_dir: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: onthego-exporter output_dir");
        }

        let output_dir = args[1].clone();

        Ok(Config { output_dir })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let uri = "mongodb://localhost:27017";
    let mongodb_repository = MongodbArticleRepository::with_uri_str(uri);

    let root_path = Path::new(&config.output_dir);
    let json_repository = JsonArticleRepository::from_path(root_path);

    for article in mongodb_repository.get_all() {
        match json_repository.save(&article) {
            Ok(()) => println!(" [Saved]"),
            Err(error) => println!(" [Error: {}]", error)
        }
    }

    Ok(())
}
