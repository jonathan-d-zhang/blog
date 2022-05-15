#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use crate::article::Article;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use std::sync::atomic::AtomicUsize;

mod article;
mod get_article;
mod submit_article;

#[get("/")]
async fn index() -> Template {
    let mut i = 0;
    let mut articles = Vec::new();
    while let Ok(article) = Article::read_article(i).await {
        articles.push((i, article));
        i += 1;
    }

    Template::render(
        "home",
        json!({"articles": articles, "summary": articles.iter().map(|a| a.1.truncate_body()).collect::<Vec<_>>()}),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                submit_article::form,
                submit_article::submit,
                get_article::article_page,
                get_article::articles
            ],
        )
        .mount("/styles", FileServer::from(relative!("styles")))
        .mount("/fonts", FileServer::from(relative!("fonts")))
        .attach(Template::fairing())
        .manage(FileCount(AtomicUsize::new(
            std::fs::read_dir("articles").unwrap().count(),
        )))
}

pub struct FileCount(AtomicUsize);
