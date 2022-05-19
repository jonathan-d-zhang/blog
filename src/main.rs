#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use crate::article::Article;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket_dyn_templates::Template;

mod article;
mod get_article;
mod submit_article;

#[get("/")]
async fn index() -> Result<Template, Status> {
    let articles = Article::read_articles()
        .await
        .map_err(|_| Status::InternalServerError)
        .unwrap();

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
}
