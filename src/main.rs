#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use rocket::serde::json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::fs;
use rocket::tokio::io::AsyncWriteExt;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rocket_dyn_templates::Template;
use std::io::Read;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

mod get_article;
mod submit_article;

#[get("/")]
async fn index(articles_data: &State<ArticlesData>) -> Template {
    let articles = articles_data.titles.lock().await;
    Template::render(
        "index",
        &json!({ "articles": articles.iter().enumerate().collect::<Vec<_>>() }),
    )
}

#[launch]
fn rocket() -> _ {
    let article_data = SerArticlesData::load();

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                submit_article::form,
                submit_article::submit,
                get_article::article
            ],
        )
        .attach(Template::fairing())
        .manage(ArticlesData {
            count: article_data.count,
            titles: article_data.titles,
        })
}

pub struct ArticlesData {
    count: AtomicU32,
    titles: Mutex<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SerArticlesData {
    count: u32,
    titles: Vec<String>,
}

impl SerArticlesData {
    fn create_articles_data(self) -> ArticlesData {
        ArticlesData {
            count: AtomicU32::new(self.count),
            titles: Mutex::new(self.titles),
        }
    }

    fn load() -> ArticlesData {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open(Path::new("articles/articles_data.json"))
            .unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let data: SerArticlesData = json::from_str(&contents).unwrap();

        data.create_articles_data()
    }
}

impl ArticlesData {
    async fn create_ser_articles_data(&self) -> SerArticlesData {
        SerArticlesData {
            count: self.count.load(Ordering::Relaxed),
            titles: self.titles.lock().await.clone(),
        }
    }

    pub async fn update(&self) -> std::io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(Path::new("articles/articles_data.json"))
            .await?;
        file.write(serde_json::to_string(&self.create_ser_articles_data().await)?.as_bytes())
            .await?;
        Ok(())
    }
}
