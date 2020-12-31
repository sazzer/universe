use crate::{
    http::{
        hal::{HalPayload, HalResponse},
        problem::{Problem, ValidationProblem, VALIDATION_PROBLEM_MISSING_FIELD},
        Response,
    },
    users::{GetUserUseCase, Username},
};
use actix_web::web::{Data, Json};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};

/// Representation of the form data to consume
#[derive(Deserialize)]
pub struct Input {
    /// The username submitted
    username: Option<String>,
}

/// HTTP handler to start authentication for a username.
///
/// # Parameters
/// - `form` - The form data containing the username
/// - `users_service` - The users service, to see if the username exists
///
/// # Returns
/// The Hal model for how to start authentication.
/// If the provided username is known to the system then returns the model for the Authentiate action.
/// If the provided username isn't known to the system then returns the model for the Register action.
///
/// If no username was provided then an RFC-7807 problem is returned indicating this.
pub async fn start(
    input: Json<Input>,
    users_service: Data<Arc<dyn GetUserUseCase>>,
) -> Result<Response<HalPayload<()>>, Problem> {
    let username = input
        .username
        .clone()
        .and_then(|username| Username::from_str(&username).ok())
        .ok_or_else(|| {
            ValidationProblem::default()
                .with_field("username", VALIDATION_PROBLEM_MISSING_FIELD)
                .build()
        })?;

    let user = users_service.get_user_by_username(&username).await;

    let mut payload = HalPayload::new(()).with_link("self", "/authentication");

    if user.is_some() {
        payload = payload.with_link(
            "tag:universe,2020:rels/authentication/authenticate",
            "/authentication/authenticate",
        )
    } else {
        payload = payload.with_link(
            "tag:universe,2020:rels/authentication/register",
            "/authentication/register",
        )
    }

    Ok(HalResponse {
        body: Some(payload),
        ..HalResponse::default()
    }
    .into())
}
