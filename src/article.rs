use rocket::serde::json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::fs;
use rocket::tokio::io::AsyncReadExt;
use std::io::Result as IoResult;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Article {
    pub title: String,
    pub body: String,
    pub time: i64,
}

impl Article {
    pub async fn read_article(n: u32) -> IoResult<Article> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(Path::new("articles").join(format!("{}.json", n)))
            .await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let article = json::from_str(&contents).unwrap();

        Ok(article)
    }

    pub fn truncate_body(&self) -> String {
        String::new()
    }
}
