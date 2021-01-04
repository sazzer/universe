use crate::{
    http::{
        hal::HalPayload,
        problem::{
            Problem, SimpleProblemType, ValidationProblem, VALIDATION_PROBLEM_MISSING_FIELD,
        },
        Response,
    },
    users::{GetUserUseCase, Username},
};
use actix_http::http::StatusCode;
use actix_web::web::{Data, Json};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};

/// Representation of the input data to consume
#[derive(Deserialize)]
pub struct Input {
    /// The username submitted
    username: Option<String>,
    /// The password submitted
    password: Option<String>,
}

/// Problem to return for an authentication failure
const AUTHENTICATION_FAILED: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:universe/2020:problems/authentication/failed",
    problem_title: "Authentication failed",
    status_code: StatusCode::UNPROCESSABLE_ENTITY,
};

/// HTTP handler to authenticate a given username and password.
///
/// # Parameters
/// - `input` - The input data containing the username and password
/// - `users_service` - The users service, to see if the username exists
///
/// # Returns
pub async fn authenticate(
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

    let password = input.password.clone().ok_or_else(|| {
        ValidationProblem::default()
            .with_field("password", VALIDATION_PROBLEM_MISSING_FIELD)
            .build()
    })?;

    let user = users_service
        .get_user_by_username(&username)
        .await
        .ok_or_else(|| Problem::from(AUTHENTICATION_FAILED))?;

    if user.data.password == password.as_ref() {
        todo!()
    } else {
        Err(Problem::from(AUTHENTICATION_FAILED))
    }
}
