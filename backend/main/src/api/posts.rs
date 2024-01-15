use salvo::prelude::*;
use super::query::{self, ErrorMessage, Post};

#[handler]
pub async fn get_posts(res: &mut Response) {
    let baseurl = std::env::var("POSTAPI").expect("POSTAPI is not definied");
    let url = format!("{baseurl}/{}","post");

    let posts = query::api_request::<Vec<Post>, ErrorMessage>(&url, None, query::HttpMethod::GET, None)
        .await;

    match posts {
        Ok(post) => {
            let status = StatusCode::from_u16(post.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(post.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }

}

#[handler]
pub async fn get_post(req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap();
    let baseurl = std::env::var("POSTAPI").expect("POSTAPI is not definied");
    let url = format!("{baseurl}/{}/{}","post", id);

    let post = query::api_request::<Post, ErrorMessage>(&url, None, query::HttpMethod::GET, None)
        .await;

    match post {
        Ok(post) => {
            let status = StatusCode::from_u16(post.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(post.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn create_post(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let baseurl = std::env::var("POSTAPI").expect("POSTAPI is not definied");
    let url = format!("{baseurl}/{}","post");
    let token = depot.jwt_auth_token().unwrap();
    let data = req.payload().await.unwrap();
    let body = std::str::from_utf8(data).unwrap().to_string();

    let post = query::api_request::<Post, ErrorMessage>(&url, Some(token), query::HttpMethod::POST, Some(&body))
        .await;

    match post {
        Ok(post) => {
            let status = StatusCode::from_u16(post.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(post.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn modify_post(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let baseurl = std::env::var("POSTAPI").expect("POSTAPI is not definied");
    let id = req.param::<String>("id").unwrap();
    let url = format!("{baseurl}/{}/{}","post", id);
    let token = depot.jwt_auth_token().unwrap();
    let data = req.payload().await.unwrap();
    let body = std::str::from_utf8(data).unwrap().to_string();

    let post = query::api_request::<Post, ErrorMessage>(&url, Some(token), query::HttpMethod::PUT, Some(&body))
        .await;

    match post {
        Ok(post) => {
            let status = StatusCode::from_u16(post.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(post.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn delete_post(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let baseurl = std::env::var("POSTAPI").expect("POSTAPI is not definied");
    let id = req.param::<String>("id").unwrap();
    let url = format!("{baseurl}/{}/{}","post", id);
    let token = depot.jwt_auth_token().unwrap();
    let data = req.payload().await.unwrap();
    let body = std::str::from_utf8(data).unwrap().to_string();

    let post = query::api_request::<query::SimpleResponse, ErrorMessage>(&url, Some(token), query::HttpMethod::DELETE, Some(&body))
        .await;

    match post {
        Ok(post) => {
            let status = StatusCode::from_u16(post.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(post.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}
