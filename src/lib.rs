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
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let output_dir = match args.next() {
            Some(arg) => arg,
            None => return Err("Usage: onthego-exporter output_dir"),
        };

        Ok(Config { output_dir })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let uri = "mongodb://localhost:27017";
    let mongodb_repository = MongodbArticleRepository::with_uri_str(uri);

    let root_path = Path::new(&config.output_dir);
    let json_repository = JsonArticleRepository::from_path(root_path);

    for article in mongodb_repository.get_all() {
        match json_repository.save(article) {
            Ok(()) => println!(" [Saved]"),
            Err(error) => println!(" [Error: {}]", error)
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_should_return_correctly_initialized_output_dir() {
        let args = [
            String::from("onthego-exporter"),
            String::from("my_output_dir"),
        ];

        let config = Config::new(&args).unwrap();

        assert_eq!(args[1], config.output_dir);
    }
}
