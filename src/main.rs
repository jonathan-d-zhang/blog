#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use std::path::Path;

mod get_blog;
mod submit_article;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            submit_article::form,
            submit_article::submit,
            get_blog::article
        ],
    )
}
