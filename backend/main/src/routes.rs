use crate::api::auth::{self, login, register, check_auth, register_profile};
use crate::api::images;
use crate::api::posts::{create_post, delete_post, get_post, get_posts, modify_post};
use crate::api::profiles::{
    delete_profile, get_my_profile, get_profile, get_profile_by_userid, get_profiles,
    update_profile,
};
use crate::api::websockets::connect;
use crate::api::users::{delete_user, get_my_user, update_user};
use salvo::hyper::Method;
// use salvo::cors::{self as cors, Cors};
// use salvo::http::Method;
use salvo::logging::Logger;
use salvo::prelude::*;
use salvo::cors::{self, Cors};

pub fn router() -> Router {
    let secret = std::env::var("SECRETKEY").expect("SECRETKEY is not definied");

    let cors_handler = Cors::new()
        .allow_origin(cors::Any)
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            "CONTENT-TYPE",
            "content-type",
            "Access-Control-Request-Method",
            "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers",
            "Access-Control-Max-Age",
            "authorization",
            "Authorization",
        ])
        .into_handler();

    let router = Router::new()
        .hoop(Logger::default())
        .hoop(cors_handler)
        .options(handler::empty())
        .hoop(auth::auth_handler(secret))
        .push(
            Router::with_path("api/v1")
                // .hoop(auth::validate_token)
                .push(Router::with_path("login").post(login))
                .push(Router::with_path("register").post(register))
                .push(
                    Router::with_path("create-profile")
                    .hoop(auth::validate_token)
                    .post(register_profile)
                )
                .push(
                    Router::with_path("checkauth")
                        .hoop(auth::validate_token)
                        .get(check_auth)
                )
                .push(
                    Router::with_path("profiles")
                        .get(get_profiles)
                        .push(
                            Router::with_path("myprofile")
                                .hoop(auth::validate_token)
                                .get(get_my_profile),
                        )
                        .push(Router::with_path("<id>").get(get_profile))
                        .push(Router::with_path("byuserid/<id>").get(get_profile_by_userid)),
                )
                .push(
                    Router::with_path("modifyprofile/<id>")
                        .hoop(auth::validate_token)
                        .put(update_profile)
                        .delete(delete_profile),
                )
                .push(
                    Router::with_path("users/myuser")
                        .hoop(auth::validate_token)
                        .get(get_my_user),
                )
                .push(
                    Router::with_path("modifyuser/<id>")
                        .hoop(auth::validate_token)
                        .put(update_user)
                        .delete(delete_user),
                )
                .push(
                    Router::with_path("posts")
                        .get(get_posts)
                        .push(Router::with_path("<id>").get(get_post)),
                )
                .push(
                    Router::with_path("modifypost")
                        .hoop(auth::validate_token)
                        .post(create_post)
                        .push(
                            Router::with_path("<id>")
                                .put(modify_post)
                                .delete(delete_post),
                        ),
                )
                .push(
                    Router::with_path("upload")
                        .hoop(auth::validate_token)
                        .post(images::upload),
                )
                .push(
                    Router::with_path("ws")
                    // .hoop(auth::validate_token)
                    .goal(connect)
                )
                .push(Router::with_path("images/<**path>").get(StaticDir::new(["data/images"]))),
        )
        .push(
            Router::with_path("<**rest>")
                // .hoop(cors_handler)
                .options(handler::empty())
        )
        ;

    router
}
