use crate::article::Article;
use crate::FileCount;
use rocket::State;
use rocket_dyn_templates::Template;
use std::io::Result as IoResult;
use std::sync::atomic::Ordering;

#[get("/articles")]
pub async fn articles(article_count: &State<FileCount>) -> IoResult<Template> {
    let t = article_count.0.load(Ordering::Relaxed);
    let articles = Article::read_articles(t as u32, article_count).await?;

    Ok(Template::render(
        "articles",
        json!({"articles": articles.into_iter().map(|article| (article.truncate_body(), article.parse_timestamp(), article)).collect::<Vec<_>>()
        }),
    ))
}

#[get("/article/<n>")]
pub async fn article_page(n: u32) -> IoResult<Template> {
    let data = Article::read_article(n).await?;

    Ok(Template::render(
        "post",
        json!({"title": data.title, "body": data.body, "time": data.parse_timestamp()}),
    ))
}
