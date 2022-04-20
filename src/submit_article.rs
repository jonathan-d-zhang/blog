use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::tokio::fs;
use rocket::tokio::io::AsyncWriteExt;
use std::path::Path;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

#[derive(FromForm, Debug)]
pub struct Article<'r> {
    title: &'r str,
    body: &'r str,
}

#[get("/form")]
pub async fn form() -> Option<NamedFile> {
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

async fn persist(article: Article<'_>) -> std::io::Result<()> {
    match file_count().await {
        Ok(n) => {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(Path::new("articles").join(format!("{}-{}.txt", n, article.title)))
                .await?;

            file.write(article.body.as_bytes()).await?;

            Ok(())
        }

        Err(e) => Err(e),
    }
}

#[post("/submit", data = "<form>")]
pub async fn submit(form: Form<Article<'_>>) -> Option<NamedFile> {
    let article = form.into_inner();
    let response = persist(article).await;

    NamedFile::open(Path::new("static").join(match response {
        Ok(_) => "success.html",
        Err(_) => "error.html",
    }))
    .await
    .ok()
}
