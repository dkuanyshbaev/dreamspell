// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use rocket::{
    response::{content, Redirect},
    State,
};
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> Template {
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/calc", format = "application/json", data = "<date>")]
async fn calc(date: String) -> content::RawJson<String> {
    content::RawJson(date)
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
