// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use rocket::{
    response::Redirect,
    serde::{json::Json, Serialize},
};
use rocket_dyn_templates::Template;

pub mod auth;
pub mod cors;
pub mod error;
pub mod tables;
pub mod tzolkin;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TzolkinData {
    kin: u32,
    archetype: (u32, u32),
}

#[get("/")]
fn home() -> Template {
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/calc", format = "application/json", data = "<date>")]
async fn calc(_key: auth::ApiKey, date: String) -> Json<TzolkinData> {
    let date_parts: [u32; 3] = date
        .split(".")
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap_or([0; 3]);

    let kin = tzolkin::kin(&date_parts);
    let archetype = tzolkin::archetype(kin);
    Json(TzolkinData { kin, archetype })
}

#[options("/<_..>")]
fn options() {}

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
        .mount("/", routes![home, calc, options])
        .register("/", catchers![not_found, internal_error])
        .attach(Template::fairing())
        .attach(cors::Cors)
}
