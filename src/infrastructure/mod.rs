mod mongodb_article_repository;
mod json_article_repository;

pub use mongodb_article_repository::MongodbArticleRepository;
pub use json_article_repository::JsonArticleRepository;
