//////////////////////////////////////////
// Dreamspell templates
//////////////////////////////////////////
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use tzolkin::Tzolkin;

#[derive(Template)]
#[template(path = "site/home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "site/home_en.html")]
pub struct HomeEnTemplate;

#[derive(Template)]
#[template(path = "site/result.html")]
pub struct ResultTemplate {
    pub result: Tzolkin,
}

#[derive(Template)]
#[template(path = "site/result_en.html")]
pub struct ResultEnTemplate {
    pub result: Tzolkin,
}

#[derive(Template)]
#[template(path = "site/oferta.html")]
pub struct OfaTemplate;

#[derive(Template)]
#[template(path = "site/oferta_en.html")]
pub struct OfaEnTemplate;

#[derive(Template)]
#[template(path = "site/howto.html")]
pub struct HowToTemplate;

#[derive(Template)]
#[template(path = "site/howto_en.html")]
pub struct HowToEnTemplate;

pub struct HtmlTemplate<T>(pub T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
