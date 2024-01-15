use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::query::Auth;

use super::query::{self, ErrorMessage, Profile};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub exp: i64,
    pub user_id: String,
}

#[derive(Debug, Serialize)]
pub struct AuthorizeState {
    message: String,
    status_code: u16,
}

#[handler]
pub async fn validate_token(depot: &mut Depot, res: &mut Response) {
    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {

            // let token = depot.jwt_auth_token().unwrap();
            // println!("TOKEN: {}", token);
        }
        JwtAuthState::Unauthorized => {
            let state = AuthorizeState {
                message: "Unauthorized".to_string(),
                status_code: 401,
            };
            res.status_code(StatusCode::from_u16(401).unwrap());
            res.render(Json(&state));
        }
        JwtAuthState::Forbidden => {
            let state = AuthorizeState {
                message: "Forbidden".to_string(),
                status_code: 403,
            };
            res.status_code(StatusCode::from_u16(403).unwrap());
            res.render(Json(&state));
        }
    }
}

pub fn auth_handler(secret_key: String) -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(secret_key.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(true)
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let baseurl = std::env::var("AUTHAPI").expect("AUTHAPI is not definied");
    let url = format!("{baseurl}/{}","auth/login");
    let data = req.payload().await.unwrap();
    let body = std::str::from_utf8(data).unwrap().to_string();

    let user = query::api_request::<Auth, ErrorMessage>(&url, None, query::HttpMethod::POST, Some(&body))
        .await;

    match user {
        Ok(user) => {
            let status = StatusCode::from_u16(user.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(user.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn register(req: &mut Request, res: &mut Response) {
    let baseurl = std::env::var("AUTHAPI").expect("AUTHAPI is not definied");
    let url = format!("{baseurl}/{}","auth/register");
    let data = req.payload().await.unwrap();
    let body = std::str::from_utf8(data).unwrap().to_string();

    let user = query::api_request::<Auth, ErrorMessage>(&url, None, query::HttpMethod::POST, Some(&body))
        .await;

    match user {
        Ok(user) => {
            let status = StatusCode::from_u16(user.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(user.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn register_profile(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let baseurl = std::env::var("USERAPI").expect("USERAPI is not definied");
    let url = format!("{baseurl}/{}","create-profile");
    let data = req.payload().await.unwrap();
    let token = depot.jwt_auth_token().unwrap();
    let body = std::str::from_utf8(data).unwrap().to_string();

    let user = query::api_request::<Profile, ErrorMessage>(&url, Some(token), query::HttpMethod::POST, Some(&body))
        .await;

    match user {
        Ok(user) => {
            let status = StatusCode::from_u16(user.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(user.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn check_auth(res: &mut Response) {
    let status: query::SimpleResponse = query::SimpleResponse { message: String::from("you are authenticated") };
    res.render(Json(status));
}
