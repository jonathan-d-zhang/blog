#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_json;

use crate::article::Article;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket_dyn_templates::Template;
use std::fs;
use std::io::Result as IoResult;
use std::path::Path;

mod article;
mod get_article;

#[cfg(debug_assertions)]
use rocket::State;
#[cfg(debug_assertions)]
mod hot_reload;

#[cfg(debug_assertions)]
#[get("/")]
async fn index(file_tracker: &State<hot_reload::FileTracker>) -> Result<Template, Status> {
    file_tracker.check_templates();
    let articles = Article::read_articles()
        .await
        .map_err(|_| Status::InternalServerError)
        .unwrap();

    Ok(Template::render(
        "home",
        json!({ "articles": articles.into_iter().take(3).collect::<Vec<_>>() }),
    ))
}

#[cfg(not(debug_assertions))]
#[get("/")]
async fn index() -> Result<Template, Status> {
    let articles = Article::read_articles()
        .await
        .map_err(|_| Status::InternalServerError)
        .unwrap();

    Ok(Template::render(
        "home",
        json!({ "articles": articles.into_iter().take(3).collect::<Vec<_>>() }),
    ))
}

#[launch]
fn rocket() -> _ {
    let _ = fs::create_dir("articles/json");

    compile_markdown().unwrap();

    let mut r = rocket::build()
        .mount(
            "/",
            routes![index, get_article::article_page, get_article::articles],
        )
        .mount("/styles", FileServer::from(relative!("styles")))
        .mount("/fonts", FileServer::from(relative!("fonts")))
        .mount("/images", FileServer::from(relative!("images")))
        .attach(Template::fairing());

    #[cfg(debug_assertions)]
    if cfg!(debug_assertions) {
        r = r.manage(hot_reload::FileTracker::new());
    }

    r
}

fn compile_markdown() -> IoResult<()> {
    println!("Compiling Markdown:");
    for entry in fs::read_dir("articles/md")? {
        let path = entry?.path();
        let html_path = Path::new("articles/json")
            .join(path.file_name().unwrap())
            .with_extension("json");
        if !html_path.exists() {
            article::compile_markdown(&path)?;
            println!("   >> Compiled {:?}", path.file_name().unwrap());
        }
    }

    println!("Finished compiling Markdown");

    Ok(())
}
