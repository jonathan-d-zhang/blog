#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use rocket_dyn_templates::Template;

mod get_article;
mod submit_article;

#[get("/")]
async fn index() -> Template {
    let mut i = 0;

    let mut articles = Vec::new();

    while let Ok(data) = get_article::read_article(i).await {
        articles.push((i, data.title));

        i += 1;
    }

    Template::render("index", &json!({ "articles": articles }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                submit_article::form,
                submit_article::submit,
                get_article::article
            ],
        )
        .attach(Template::fairing())
}
