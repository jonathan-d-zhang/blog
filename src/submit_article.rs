use crate::ArticlesData;
use chrono::Utc;
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::fs;
use rocket::tokio::io::AsyncWriteExt;
use rocket::State;
use rocket_dyn_templates::Template;
use scrypt::{
    password_hash::{PasswordHash, PasswordVerifier},
    Scrypt,
};
use std::env;
use std::io::Result as IoResult;
use std::path::Path;
use std::sync::atomic::Ordering;

#[derive(FromForm, Debug)]
pub struct ArticleForm {
    pub article: Article,
    pub password: String,
}

#[derive(FromForm, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Article {
    pub title: String,
    pub body: String,
}

impl Article {
    fn create_ser_article(self) -> SerArticle {
        SerArticle {
            title: self.title,
            body: self.body,
            time: Utc::now().timestamp(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SerArticle {
    pub title: String,
    pub body: String,
    pub time: i64,
}

#[get("/form")]
pub async fn form() -> Template {
    Template::render("upload_form", &json!({"wrong": false}))
}

async fn persist(article: Article, articles_data: &State<ArticlesData>) -> IoResult<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(Path::new("articles").join(format!(
            "{}.json",
            articles_data.count.fetch_add(1, Ordering::Relaxed)
        )))
        .await?;

    articles_data
        .titles
        .lock()
        .await
        .push(article.title.clone());
    articles_data.update().await?;

    file.write(serde_json::to_string(&article.create_ser_article())?.as_bytes())
        .await?;

    Ok(())
}

async fn check_password(password: &str) -> Result<bool, Status> {
    let password_hash = env::var("PASSWORD_HASH").map_err(|_| Status::InternalServerError)?;
    Ok(Scrypt
        .verify_password(
            password.as_bytes(),
            &PasswordHash::new(&password_hash).unwrap(),
        )
        .is_ok())
}

#[post("/form", data = "<form>")]
pub async fn submit(
    form: Form<ArticleForm>,
    articles_data: &State<ArticlesData>,
) -> Result<Template, Status> {
    let r = form.into_inner();
    let article = r.article;

    if !check_password(&r.password).await? {
        return Ok(Template::render(
            "upload_form",
            &json!(
                {
                    "wrong": true,
                    "title": article.title,
                    "body": article.body
                }
            ),
        ));
    }

    let response = persist(article.clone(), articles_data).await;

    match response {
        Ok(_) => Ok(Template::render("success", &json!({"name": article.title}))),

        Err(_) => Err(Status::InternalServerError),
    }
}
