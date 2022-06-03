use crate::article::Article;
use rocket_dyn_templates::Template;
use std::io::Result as IoResult;

#[cfg(debug_assertions)]
use crate::hot_reload::FileTracker;
#[cfg(debug_assertions)]
use rocket::State;

#[cfg(not(debug_assertions))]
#[get("/articles")]
pub async fn articles() -> IoResult<Template> {
    let articles = Article::read_articles().await?;

    Ok(Template::render(
        "articles",
        json!({ "articles": articles }),
    ))
}

#[cfg(debug_assertions)]
#[get("/articles")]
pub async fn articles(file_tracker: &State<FileTracker>) -> IoResult<Template> {
    file_tracker.check_templates();

    let articles = Article::read_articles().await?;

    Ok(Template::render(
        "articles",
        json!({ "articles": articles }),
    ))
}

#[cfg(not(debug_assertions))]
#[get("/articles/<path>")]
pub async fn article_page(path: &str) -> IoResult<Template> {
    let data = Article::read_article(path).await?;

    Ok(Template::render("post", json!({ "article": data })))
}

#[cfg(debug_assertions)]
#[get("/articles/<path>")]
pub async fn article_page(path: &str, file_tracker: &State<FileTracker>) -> IoResult<Template> {
    file_tracker.check_templates();
    if file_tracker.check_articles() {
        println!("Detected changes in article, recompiling markdown");
        use std::fs::{create_dir, remove_dir_all};
        remove_dir_all("articles/json").unwrap();
        create_dir("articles/json").unwrap();
        super::compile_markdown().unwrap();
    }

    let data = Article::read_article(path).await?;

    Ok(Template::render("post", json!({ "article": data })))
}
