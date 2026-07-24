use serde::Serialize;

use crate::BEARER;

#[derive(Serialize)]
#[cfg_attr(test, derive(serde::Deserialize, Debug))]
pub struct TokenResponse {
    access_token: String,
    id_token: String,
    scope: String,
    expires_in: i32,
    token_type: String,
    // Auth0 returns a refresh_token on authorization_code exchanges when
    // offline_access is requested. CleverTap's dashboard AuthFilter treats an
    // Auth0 session without a refresh token as expired, so it must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
}

impl TokenResponse {
    pub fn new(
        access_token: String,
        id_token: String,
        scope_opt: Option<String>,
        refresh_token: Option<String>,
    ) -> Self {
        Self {
            access_token,
            id_token,
            scope: scope_opt.unwrap_or_default(),
            expires_in: 86400,
            token_type: BEARER.to_string(),
            refresh_token,
        }
    }

    #[cfg(test)]
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    #[cfg(test)]
    pub fn id_token(&self) -> &str {
        &self.id_token
    }

    pub fn response_types_supported() -> Vec<String> {
        vec!["token id_token".to_string()]
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub code: String,
}
