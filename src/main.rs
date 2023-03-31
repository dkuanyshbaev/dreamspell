// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use crate::tzolkin::{Seals, Tzolkin};
use rocket::{
    response::{status, Redirect},
    serde::json::Json,
    State,
};
use rocket_dyn_templates::Template;
use std::{fs, process};

pub mod auth;
pub mod config;
pub mod cors;
pub mod descriptions;
pub mod error;
pub mod tables;
pub mod tzolkin;

#[macro_use]
extern crate rocket;

const SEALS: &str = "resources/seals.json";

#[get("/")]
fn home() -> Template {
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/calc", format = "application/json", data = "<date>")]
async fn calc(
    config: &State<config::Config>,
    seals: &State<Seals>,
    key: auth::ApiKey,
    date: String,
) -> Result<Json<Tzolkin>, status::Unauthorized<()>> {
    if !config.api_key.eq(&key.0) {
        Err(status::Unauthorized::<()>(None))
    } else {
        Ok(Json(Tzolkin::calc(
            seals.inner(),
            &date
                .split(".")
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap_or([0; 3]),
        )))
    }
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
    let config = config::Config::new().unwrap_or_else(|_| {
        println!("APIKEY in env!");
        process::exit(1);
    });

    let seals = {
        let seals = fs::read_to_string(&SEALS).expect("Can't find seals file");
        serde_json::from_str::<Seals>(&seals).expect("Can't parse seals file")
    };

    rocket::build()
        .mount("/", routes![home, calc, options])
        .register("/", catchers![not_found, internal_error])
        .attach(Template::fairing())
        .attach(cors::Cors)
        .manage(config)
        .manage(seals)
}
