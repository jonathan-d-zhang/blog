use crate::article::Article;
use chrono::{DateTime, NaiveDateTime, Utc};
use pulldown_cmark::{html, Parser};
use rocket_dyn_templates::Template;
use std::io::Result as IoResult;

#[get("/article")]
pub async fn articles() -> IoResult<Template> {
    Ok(Template::render("articles", json!({})))
}

#[get("/article/<n>")]
pub async fn article_page(n: u32) -> IoResult<Template> {
    let data = Article::read_article(n).await?;
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(data.time, 0), Utc);

    let parser = Parser::new(&data.body);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(Template::render(
        "article",
        json!({"title": data.title, "body": html_output, "time": format!("{}", dt.format("%B %e, %Y"))}),
    ))
}
