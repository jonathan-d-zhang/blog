use crate::submit_article::Article;
use rocket::serde::json;
use rocket::tokio::fs;
use rocket::tokio::io::AsyncReadExt;
use std::io::Result as IoResult;
use std::path::Path;

pub async fn read_article(n: u32) -> IoResult<Article> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open(Path::new("articles").join(format!("{}.txt", n)))
        .await?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let article = json::from_str(&contents).unwrap();

    Ok(article)
}

#[get("/article/<n>")]
pub async fn article(n: u32) -> IoResult<String> {
    read_article(n).await.map(|a| a.body)
}