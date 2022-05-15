use crate::submit_article::SerArticle;
use chrono::{DateTime, NaiveDateTime, Utc};
use pulldown_cmark::{html, Parser};
use rocket::serde::json;
use rocket::tokio::fs;
use rocket::tokio::io::AsyncReadExt;
use rocket_dyn_templates::Template;
use std::io::Result as IoResult;
use std::path::Path;

async fn read_article(n: u32) -> IoResult<SerArticle> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open(Path::new("articles").join(format!("{}.json", n)))
        .await?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let article = json::from_str(&contents).unwrap();

    Ok(article)
}

#[get("/article/<n>")]
pub async fn article(n: u32) -> IoResult<Template> {
    let data = read_article(n).await?;
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(data.time, 0), Utc);

    let parser = Parser::new(&data.body);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(Template::render(
        "article",
        json!({"title": data.title, "body": html_output, "time": format!("{}", dt.format("%B %e, %Y"))}),
    ))
}
