// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use rocket::{form::Form, response::Redirect, State};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct FormData {
    question: String,
}

#[get("/")]
fn home() -> Template {
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/question", data = "<form_data>")]
async fn calc(form_data: Form<FormData>) -> Redirect {
    Redirect::to("/")
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}

#[catch(500)]
pub fn internal_error() -> Redirect {
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home, calc])
        .register("/", catchers![not_found, internal_error])
        .attach(Template::fairing())
}
