use super::Article;

pub trait ArticleRepository {
    fn get_all(&self) -> Vec<Article>;
}
