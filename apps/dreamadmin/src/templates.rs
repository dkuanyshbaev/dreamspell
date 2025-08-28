//////////////////////////////////////////
// Dreamadmin templates
//////////////////////////////////////////
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use tzolkin::Seal;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "admin.html")]
pub struct AdminTemplate {
    pub seals: Vec<Seal>,
}

#[derive(Template)]
#[template(path = "seal.html")]
pub struct SealDetailTemplate {
    pub seal: Seal,
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!(error = %err, "Failed to render template");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                )
                    .into_response()
            }
        }
    }
}