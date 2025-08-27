//////////////////////////////////////////
// Dreamadmin views
//////////////////////////////////////////
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
};

use crate::auth::{AuthSession, Credentials};
use crate::templates::{HtmlTemplate, LoginTemplate, AdminTemplate};

pub async fn admin() -> HtmlTemplate<AdminTemplate> {
    tracing::info!("Admin dashboard accessed");
    HtmlTemplate(AdminTemplate)
}

pub async fn login_get() -> HtmlTemplate<LoginTemplate> {
    HtmlTemplate(LoginTemplate { error: None })
}

pub async fn login_post(
    mut auth_session: AuthSession,
    Form(credentials): Form<Credentials>,
) -> impl IntoResponse {
    tracing::info!(
        username = %credentials.username,
        "Login attempt"
    );
    
    match auth_session.authenticate(credentials.clone()).await {
        Ok(Some(user)) => {
            if auth_session.login(&user).await.is_ok() {
                tracing::info!(
                    username = %credentials.username,
                    "Successful login"
                );
                Redirect::to("/admin").into_response()
            } else {
                tracing::error!(
                    username = %credentials.username,
                    "Session login failed after successful authentication"
                );
                (StatusCode::INTERNAL_SERVER_ERROR, "Login failed").into_response()
            }
        }
        Ok(None) => {
            tracing::warn!(
                username = %credentials.username,
                "Failed login attempt: invalid credentials"
            );
            let template = HtmlTemplate(LoginTemplate {
                error: Some("Invalid username or password".to_string()),
            });
            template.into_response()
        }
        Err(e) => {
            tracing::error!(
                username = %credentials.username,
                error = ?e,
                "Authentication system error"
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response()
        }
    }
}


pub async fn nothing() -> impl IntoResponse {
    tracing::warn!("404 Not Found - unknown route requested");
    (StatusCode::NOT_FOUND, "Not found")
}

