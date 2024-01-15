use salvo::prelude::*;
use super::query;

#[handler]
pub async fn get_my_profile(depot: &mut Depot, res: &mut Response) {
    let baseurl = std::env::var("AUTHAPI").expect("AUTH is not definied");
    let token = depot.jwt_auth_token().unwrap();
    let url = format!("{baseurl}/{}","auth/profile");

    let profile = query::api_request::<query::Profile, query::ErrorMessage>(&url, Some(token), query::HttpMethod::GET, None)
        .await;

    match profile {
        Ok(profile) => {
            let status = StatusCode::from_u16(profile.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(profile.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}


#[handler]
pub async fn get_profiles(res: &mut Response) {
    let baseurl = std::env::var("USERAPI").expect("USERAPI is not definied");
    let url = format!("{baseurl}/{}","profiles");

    let profiles = query::api_request::<query::Profiles, query::ErrorMessage>(&url, None, query::HttpMethod::GET, None)
        .await;

    match profiles {
        Ok(profile) => {
            let status = StatusCode::from_u16(profile.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(profile.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn get_profile(req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap();
    let baseurl = std::env::var("USERAPI").expect("USERAPI is not definied");
    let url = format!("{baseurl}/{}/{}","profiles", id);

    let profile = query::api_request::<query::Profile, query::ErrorMessage>(&url, None, query::HttpMethod::GET, None)
        .await;

    match profile {
        Ok(profile) => {
            let status = StatusCode::from_u16(profile.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(profile.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn get_profile_by_userid(req: &mut Request, res: &mut Response) {
    let id = req.param::<String>("id").unwrap();
    let baseurl = std::env::var("USERAPI").expect("USERAPI is not definied");
    let url = format!("{baseurl}/{}/{}","getprofile", id);

    let profile = query::api_request::<query::Profile, query::ErrorMessage>(&url, None, query::HttpMethod::GET, None)
        .await;

    match profile {
        Ok(profile) => {
            let status = StatusCode::from_u16(profile.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(profile.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn update_profile(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let baseurl = std::env::var("USERAPI").expect("USERAPI is not definied");
    let id = req.param::<String>("id").unwrap();
    let url = format!("{baseurl}/{}/{}","profiles", id);
    let token = depot.jwt_auth_token().unwrap();
    let data = req.payload().await.unwrap();
    let body = std::str::from_utf8(data).unwrap().to_string();

    let profile = query::api_request::<query::Profile, query::ErrorMessage>(&url, Some(token), query::HttpMethod::PUT, Some(&body))
        .await;

    match profile {
        Ok(profile) => {
            let status = StatusCode::from_u16(profile.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(profile.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}

#[handler]
pub async fn delete_profile(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let baseurl = std::env::var("USERAPI").expect("USERAPI is not definied");
    let id = req.param::<String>("id").unwrap();
    let url = format!("{baseurl}/{}/{}","profiles", id);
    let token = depot.jwt_auth_token().unwrap();
    let data = req.payload().await.unwrap();
    let body = std::str::from_utf8(data).unwrap().to_string();

    let profile = query::api_request::<query::SimpleResponse, query::ErrorMessage>(&url, Some(token), query::HttpMethod::DELETE, Some(&body))
        .await;

    match profile {
        Ok(profile) => {
            let status = StatusCode::from_u16(profile.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(profile.0))
        },
        Err(err) => {
            let status = StatusCode::from_u16(err.1.as_u16()).unwrap();
            res.status_code(status);
            res.render(Json(err.0))
        }
    }
}
