#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::tokio::{fs, io::AsyncWriteExt};
use std::path::Path;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[derive(FromForm, Debug)]
struct Article<'r> {
    title: &'r str,
    body: &'r str,
}

#[get("/form")]
async fn form() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/upload_form.html"))
        .await
        .ok()
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

#[post("/submit", data = "<form>")]
async fn submit(form: Form<Article<'_>>) -> &'static str {
    let article = form.into_inner();

    match file_count().await {
        Ok(n) => {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(Path::new("articles").join(format!("{}-{}.txt", n, article.title)))
                .await
                .unwrap();

            file.write(article.body.as_bytes()).await.unwrap();
        }

        Err(_) => panic!("Error getting file count"),
    }

    "yo"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, form, submit])
}