use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;

use crate::{application::services::authentication::AuthenticationService, config::{config_loader::get_stage, stage::Stage}, domain::repositories::{admin::AdminRepository, user::UserRepository}, infrastructure::{jwt_authentication::authentication_model::LoginModel, postgres::{postgres_connection::PgPoolSquad, repositories::{admin::AdminPostgres, user::UserPostgres}}}};


pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let user_repository = UserPostgres::new(Arc::clone(&db_pool));
    let admin_repository = AdminPostgres::new(Arc::clone(&db_pool));
    let authentication_service = AuthenticationService::new(
        Arc::new(user_repository),
        Arc::new(admin_repository),
    );

    Router::new()
        .route("/users/login", post(user_login))
        .route(
            "/users/refresh-token",
            post(user_refresh_token),
        )
        .route("/admin/login", post(admin_login))
        .route(
            "/admin/refresh-token",
            post(admin_refresh_token),
        )
        .with_state(Arc::new(authentication_service))
}

pub async fn user_login<T1, T2>(
    State(authentication_service): State<Arc<AuthenticationService<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T1: UserRepository + Send + Sync,
    T2: AdminRepository + Send + Sync,
{
    match authentication_service.user_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                rft_cookie = rft_cookie.secure(true);
                act_cookie = act_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successfully").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn user_refresh_token<T1, T2>(
    State(authentication_service): State<Arc<AuthenticationService<T1, T2>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T1: UserRepository + Send + Sync,
    T2: AdminRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();

        let response = match authentication_service
            .user_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    rft_cookie = rft_cookie.secure(true);
                    act_cookie = act_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Refresh token successfully").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        };

        return response;
    }

    (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
}

pub async fn admin_login<T1, T2>(
    State(authentication_service): State<Arc<AuthenticationService<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T1: UserRepository + Send + Sync,
    T2: AdminRepository + Send + Sync,
{
    match authentication_service
        .admin_login(login_model)
        .await
    {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                rft_cookie = rft_cookie.secure(true);
                act_cookie = act_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successfully").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn admin_refresh_token<T1, T2>(
    State(authentication_service): State<Arc<AuthenticationService<T1, T2>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T1: UserRepository + Send + Sync,
    T2: AdminRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();

        let response = match authentication_service
            .admin_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    rft_cookie = rft_cookie.secure(true);
                    act_cookie = act_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Refresh token successfully").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        };

        return response;
    }

    (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
}