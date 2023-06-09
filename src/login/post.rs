use crate::authentication::password::{validate_credentials, Credentials};
use actix_identity::{Identity, IdentityMiddleware};
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use actix_web::{web, Responder};
use secrecy::Secret;
use sqlx::PgPool;
use std::error::Error;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[derive(serde::Deserialize)]
pub struct user {
    pub(crate) username: String,
    pub(crate) password: String,
}

//#[tracing::instrument(
//skip(form, pool),
//fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
//)]
pub async fn login(form: web::Form<FormData>) -> Result<HttpResponse, actix_web::Error> {
    tracing::info!("abc");
    /* let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            /* session
            .insert_user_id(user_id)
            .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;*/
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/admin"))
                .finish())
        }
        Err(inner) => {
            todo!()
        }
    }*/

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish())
}

pub async fn logout(id: Identity) -> impl Responder {
    id.logout();

    //web::Redirect::to("/").using_status_code(StatusCode::FOUND)
    web::Redirect::to("/")
}

pub async fn check_user(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        web::Redirect::to("/admin?page=1&limit=2")
    } else {
        web::Redirect::to("/")
    }
}

/*fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}

#[derive(thiserror::Error)]
#[derive(Debug)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}*/
