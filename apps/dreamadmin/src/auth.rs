//////////////////////////////////////////
// Dreamadmin authentication
//////////////////////////////////////////
use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};

pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: i64,
    pw_hash: Vec<u8>,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash
    }
}

#[derive(Clone)]
pub struct Backend {
    secret: String,
}

impl Backend {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        if credentials.username == "admin" && credentials.password == self.secret {
            let user = User {
                id: 1,
                pw_hash: self.secret.as_bytes().to_vec(),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        if *user_id == 1 {
            let user = User {
                id: 1,
                pw_hash: self.secret.as_bytes().to_vec(),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}
