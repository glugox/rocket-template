use jsonwebtoken as jwt;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::{Outcome, State};
use serde::{Deserialize, Serialize};

use crate::config;
use crate::config::AppState;

pub struct ApiKey(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str, secret: &[u8]) -> bool {
    decode_token(key, secret).is_some()
}

#[cfg_attr(tarpaulin, skip)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    pub id: i32,
}

impl Auth {
    pub fn token(&self, secret: &[u8]) -> String {
        jwt::encode(&jwt::Header::default(), self, secret).expect("jwt")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    /// Extract Auth token from the "Authorization" header.
    ///
    /// Handlers with Auth guard will fail with 503 error.
    /// Handlers with Option<Auth> will be called with None.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        let state: State<AppState> = request.guard()?;
        if let Some(auth) = extract_auth_from_request(request, &state.secret) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
        let state: State<AppState> = request.guard()?;
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ())),
            1 if is_valid(keys[0], &state.secret) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ())),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    use jwt::{Algorithm, Validation};

    jwt::decode(token, secret, &Validation::new(Algorithm::HS256))
        .map_err(|err| {
            eprintln!("Auth decode error: {:?}", err);
        })
        .ok()
        .map(|token_data| token_data.claims)
}
