use crate::article::Article;
use rocket_dyn_templates::Template;
use std::io::Result as IoResult;

#[get("/article")]
pub async fn articles() -> IoResult<Template> {
    Ok(Template::render("articles", json!({})))
}

#[get("/article/<n>")]
pub async fn article_page(n: u32) -> IoResult<Template> {
    let data = Article::read_article(n).await?;

    Ok(Template::render(
        "article",
        json!({"title": data.title, "body": data.body, "time": data.parse_timestamp()}),
    ))
}
