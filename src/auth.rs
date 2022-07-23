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

#[derive(Debug)]
pub struct ApiKey(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = DreamspellError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiKey, Self::Error> {
        let api_key = request.headers().get_one("Api-Key");
        match api_key {
            Some(api_key) => Outcome::Success(ApiKey(api_key.to_string())),
            None => Outcome::Failure((Status::Unauthorized, DreamspellError::Unauthorized)),
        }
    }
}
