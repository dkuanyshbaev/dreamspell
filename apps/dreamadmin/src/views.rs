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
    tracing::info!("Login attempt");
    
    match auth_session.authenticate(credentials.clone()).await {
        Ok(Some(user)) => {
            if auth_session.login(&user).await.is_ok() {
                tracing::info!("Successful admin login");
                Redirect::to("/admin").into_response()
            } else {
                tracing::error!("Session login failed after successful authentication");
                (StatusCode::INTERNAL_SERVER_ERROR, "Login failed").into_response()
            }
        }
        Ok(None) => {
            tracing::warn!("Failed admin login attempt: invalid password");
            let template = HtmlTemplate(LoginTemplate {
                error: Some("Invalid password".to_string()),
            });
            template.into_response()
        }
        Err(e) => {
            tracing::error!(
                error = ?e,
                "Authentication system error"
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "Authentication error").into_response()
        }
    }
}

pub async fn root_redirect() -> impl IntoResponse {
    Redirect::to("/login")
}

pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    tracing::info!("Admin logout");
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(_) => {
            tracing::error!("Failed to logout");
            Redirect::to("/login").into_response()
        }
    }
}

pub async fn nothing() -> impl IntoResponse {
    tracing::warn!("404 Not Found - unknown route requested");
    (StatusCode::NOT_FOUND, "Not found")
}

