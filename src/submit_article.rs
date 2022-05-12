use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::fs;
use rocket::tokio::io::AsyncWriteExt;
use rocket_dyn_templates::Template;
use scrypt::{
    password_hash::{PasswordHash, PasswordVerifier},
    Scrypt,
};
use std::env;
use std::io::Result as IoResult;
use std::path::Path;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

#[derive(FromForm, Debug)]
pub struct ArticleForm {
    pub article: Article,
    pub password: String,
}

#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Article {
    pub title: String,
    pub body: String,
}

#[get("/form")]
pub async fn form() -> Template {
    Template::render("upload_form", &json!({"wrong": false}))
}

async fn file_count() -> std::io::Result<u32> {
    let stream = fs::read_dir(Path::new("articles"))
        .await
        .map(|dirs| ReadDirStream::new(dirs).map(|_| 1));

    match stream {
        Ok(mut s) => {
            let mut c = 0;
            while s.next().await.is_some() {
                c += 1;
            }

            Ok(c)
        }
        Err(e) => Err(e),
    }
}

async fn persist(article: Article) -> IoResult<()> {
    match file_count().await {
        Ok(n) => {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(Path::new("articles").join(format!("{}.json", n)))
                .await?;

            file.write(serde_json::to_string(&article)?.as_bytes())
                .await?;

            Ok(())
        }

        Err(e) => Err(e),
    }
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
pub async fn submit(form: Form<ArticleForm>) -> Result<Template, Status> {
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

    let response = persist(article.clone()).await;

    match response {
        Ok(_) => Ok(Template::render("success", &json!({"name": article.title}))),

        Err(_) => Err(Status::InternalServerError),
    }
}
