use crate::article::Article;
use rocket_dyn_templates::Template;
use std::io::Result as IoResult;

#[get("/articles")]
pub async fn articles() -> IoResult<Template> {
    let articles = Article::read_articles().await?;

    Ok(Template::render(
        "articles",
        json!({ "articles": articles }),
    ))
}

#[get("/article/<path>")]
pub async fn article_page(path: &str) -> IoResult<Template> {
    let data = Article::read_article(path).await?;

    Ok(Template::render("post", json!({ "article": data })))
}
