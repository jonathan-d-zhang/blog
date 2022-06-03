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
use std::io::Result as IoResult;
use std::path::Path;

mod article;
mod get_article;

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

#[cfg(debug_assertions)]
fn hot_reload_articles() {
    println!("Removing json directory.");
    let _ = std::fs::remove_dir_all("articles/json").unwrap();
}

#[launch]
fn rocket() -> _ {
    let _ = std::fs::create_dir("articles/json");

    #[cfg(debug_assertions)]
    hot_reload_articles();

    compile_markdown().unwrap();

    rocket::build()
        .mount(
            "/",
            routes![index, get_article::article_page, get_article::articles],
        )
        .mount("/styles", FileServer::from(relative!("styles")))
        .mount("/fonts", FileServer::from(relative!("fonts")))
        .mount("/images", FileServer::from(relative!("images")))
        .attach(Template::fairing())
}

fn compile_markdown() -> IoResult<()> {
    println!("Compiling Markdown:");
    for entry in std::fs::read_dir("articles/md")? {
        let path = entry?.path();
        let html_path = Path::new("articles/json")
            .join(path.file_name().unwrap())
            .with_extension("json");
        if !html_path.exists() {
            article::compile_markdown(&path)?;
            println!("   >> Compiled {:?}", path.file_name().unwrap());
        }
    }

    Ok(())
}
