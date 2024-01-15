use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub edited: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Users {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub id: i32,
    pub username: String,
    pub avatar: String,
    pub user_id: String,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    pub profiles: Vec<Profile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub user: User,
    pub profile: Profile
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleResponse {
    pub message: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub error: String
}

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub async fn api_request<T, E>(
    url: &str,
    token: Option<&str>,
    method: HttpMethod,
    body_data: Option<&str>,
) -> Result<(T, StatusCode), (E, StatusCode)>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let mut request_builder = match method {
        HttpMethod::GET => client.get(url),
        HttpMethod::POST => client.post(url),
        HttpMethod::PUT => client.put(url),
        HttpMethod::DELETE => client.delete(url),
    };

    request_builder = request_builder.header(ACCEPT, "application/json");

    if let Some(token) = token {
        request_builder = request_builder.header(AUTHORIZATION, format!("Bearer {}", token));
    }

    if let Some(body_data) = body_data {
        request_builder = request_builder.header(CONTENT_TYPE, "application/json");
        request_builder = request_builder.body(body_data.to_owned());
    }

    let response = request_builder.send().await.unwrap();
    let status = response.status();

    if response.status().is_success() {
        Ok((response.json::<T>().await.unwrap(), status))
    } else {
        Err((response.json::<E>().await.unwrap(), status))
    }
}
