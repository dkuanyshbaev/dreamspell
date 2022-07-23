// ---------------------------------------
// Dreamspell auth basic system
// ---------------------------------------
use crate::error::DreamspellError;
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};

const APIKEY: &str = "tzolkin";

#[derive(Debug)]
pub struct ApiKey(String);

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
