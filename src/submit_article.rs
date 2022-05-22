use crate::article::Article;
use chrono::Utc;
use pulldown_cmark::{html, Options, Parser};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::Serialize;
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

#[derive(FromForm, Debug)]
pub struct ArticleForm {
    pub title_body: TitleBody,
    pub password: String,
}

#[derive(FromForm, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TitleBody {
    pub title: String,
    pub body: String,
}

impl TitleBody {
    fn create_article(self) -> Article {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&self.body, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Article::new(self.title, html_output, Utc::now().timestamp())
    }
}

#[get("/form")]
pub async fn form() -> Template {
    Template::render("upload_form", &json!({"wrong": false}))
}

async fn persist(article: TitleBody) -> IoResult<()> {
    let article = article.create_article();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(Path::new("articles").join(format!("{}.json", article.filename)))
        .await?;

    // can only write 16 * 1024 bytes at once, maybe implement chunking later
    // for now, assume i'll never write that long of an article
    file.write(serde_json::to_string(&article)?.as_bytes())
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
pub async fn submit(form: Form<ArticleForm>) -> Result<Template, Status> {
    let r = form.into_inner();
    let title_body = r.title_body;

    if !check_password(&r.password).await? {
        return Ok(Template::render(
            "upload_form",
            &json!(
                {
                    "wrong": true,
                    "title": title_body.title,
                    "body": title_body.body
                }
            ),
        ));
    }

    let response = persist(title_body.clone()).await;

    match response {
        Ok(_) => Ok(Template::render(
            "success",
            &json!({"name": title_body.title}),
        )),

        Err(_) => Err(Status::InternalServerError),
    }
}
