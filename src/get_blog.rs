use crate::submit_article::Article;
use rocket::serde::json;
use rocket::tokio::fs;
use rocket::tokio::io::AsyncReadExt;
use std::io::Error;
use std::path::Path;

async fn read_article(n: u32) -> std::io::Result<Article> {
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
pub async fn article(n: u32) -> Result<String, Error> {
    let article = read_article(n).await;

    match article {
        Ok(a) => Ok(a.body),

        Err(e) => Err(e),
    }
}
