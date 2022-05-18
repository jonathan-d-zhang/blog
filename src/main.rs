#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use crate::article::Article;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket_dyn_templates::Template;
use std::sync::atomic::AtomicUsize;

mod article;
mod get_article;
mod submit_article;

#[get("/")]
async fn index() -> Result<Template, Status> {
    let mut articles = Vec::new();
    for article in Article::read_articles(3)
        .await
        .map_err(|_| Status::InternalServerError)
        .unwrap()
    {
        articles.push((article.truncate_body(), article.parse_timestamp(), article));
    }

    Ok(Template::render("home", json!({ "articles": articles })))
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
