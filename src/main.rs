// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    outcome::Outcome,
    request::{self, FromRequest},
    response::{content, Redirect},
    Request, Response,
};
use rocket_dyn_templates::Template;

const APIKEY: &str = "tzolkin";

#[macro_use]
extern crate rocket;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Debug)]
pub enum DreamspellError {
    Unauthorized,
}

#[derive(Debug)]
struct ApiKey(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = DreamspellError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiKey, Self::Error> {
        let api_key = request.headers().get_one("Api-Key");
        match api_key {
            Some(api_key) => {
                if api_key.eq(&APIKEY.to_string()) {
                    Outcome::Success(ApiKey(api_key.to_string()))
                } else {
                    Outcome::Failure((Status::Unauthorized, DreamspellError::Unauthorized))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, DreamspellError::Unauthorized)),
        }
    }
}

#[get("/")]
fn home() -> Template {
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/calc", format = "application/json", data = "<date>")]
async fn calc(_key: ApiKey, date: String) -> content::RawJson<String> {
    content::RawJson(date)
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
        .attach(Cors)
}
