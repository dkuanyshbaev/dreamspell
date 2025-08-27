use axum::response::IntoResponse;

use crate::auth::Credentials;
use crate::*;

pub async fn admin() -> Html<&'static str> {
    Html("<h1>Admin</h1>")
}

pub async fn login_get() -> Html<&'static str> {
    Html("<h1>Login</h1>")
}

pub async fn login_post() -> Html<&'static str> {
    Html("")
}

pub async fn login(
    mut auth_session: crate::auth::AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/admin").into_response()
}

// use crate::error::DreamspellError;
pub async fn nothing() -> impl IntoResponse {
    // DreamspellError::NotFound
    (StatusCode::NOT_FOUND, "Not found")
}
