// ---------------------------------------
// Dreamspell server
// ---------------------------------------
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Status},
    outcome::Outcome,
    request::{self, FromRequest},
    response::Redirect,
    serde::{json::Json, Serialize},
    Request, Response,
};
use rocket_dyn_templates::Template;

// type DreamspellResult<T> = Result<T, DreamspellError>;

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
    Other,
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Result {
    kin: u32,
}

// impl<'r> Responder<'r, 'r> for DreamspellError {
//     fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
//         match self {
//             DreamspellError::Unauthorized => Err(Status::Unauthorized),
//             _ => Err(Status::InternalServerError),
//         }
//     }
// }

#[get("/")]
fn home() -> Template {
    Template::render("home", rocket_dyn_templates::context! {})
}

#[post("/calc", format = "application/json", data = "<date>")]
async fn calc(date: String) -> Json<Result> {
    // async fn calc(_key: ApiKey, date: String) -> content::RawJson<String> {

    let date_parts: [u32; 3] = date
        .split(".")
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap_or([0; 3]);

    Json(Result {
        kin: kin(&date_parts),
    })
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

const MONTH_TABLE: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 13, 44, 74];
const YEAR_TABLE: [u32; 52] = [
    232, 77, 182, 27, 132, 237, 82, 187, 32, 137, 242, 87, 192, 37, 142, 247, 92, 197, 42, 147,
    252, 97, 202, 47, 152, 257, 102, 207, 52, 157, 2, 107, 212, 57, 162, 7, 112, 217, 62, 167, 12,
    117, 222, 67, 172, 17, 122, 227, 72, 177, 22, 127,
];

fn kin(parts: &[u32; 3]) -> u32 {
    let (day, month, year) = (parts[0], parts[1], parts[2]);

    if day == 0 || month == 0 || year == 0 {
        return 0;
    }

    let year_index = year as f32 - ((year as f32 / 52_f32).floor() * 52_f32);
    let mut kin = day + MONTH_TABLE[month as usize - 1] + YEAR_TABLE[year_index as usize];

    if kin > 260 {
        kin = kin - 260
    }
    kin
}
