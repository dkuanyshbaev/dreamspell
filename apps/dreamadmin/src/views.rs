use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Form,
};

use crate::auth::{AuthSession, Credentials};

#[derive(Template)]
#[template(path = "admin.html")]
pub struct AdminTemplate;

pub async fn admin() -> AdminTemplate {
    AdminTemplate
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

pub async fn login_get() -> LoginTemplate {
    LoginTemplate { error: None }
}

pub async fn login_post(
    mut auth_session: AuthSession,
    Form(credentials): Form<Credentials>,
) -> impl IntoResponse {
    match auth_session.authenticate(credentials).await {
        Ok(Some(user)) => {
            if auth_session.login(&user).await.is_ok() {
                Redirect::to("/admin").into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, "Login failed").into_response()
            }
        }
        Ok(None) => {
            let template = LoginTemplate {
                error: Some("Invalid username or password".to_string()),
            };
            template.into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response(),
    }
}


pub async fn nothing() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

impl IntoResponse for LoginTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
        }
    }
}

impl IntoResponse for AdminTemplate {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
        }
    }
}
