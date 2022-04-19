#[macro_use] extern crate rocket;
use std::path::{Path};
use rocket::fs::{NamedFile};
use rocket::form::Form;


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
    NamedFile::open(Path::new("static/upload_form.html")).await.ok()
}

#[post("/submit", data = "<form>")]
fn submit(form: Form<Article<'_>>) -> &'static str {
    println!("{:?}", form.into_inner());
    "yo"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, form, submit])
}