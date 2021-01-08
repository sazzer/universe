use super::model::AuthenticatedUserResponse;
use crate::{
    authentication::{RegisterError, RegisterUserUseCase},
    http::{
        hal::HalPayload,
        problem::{
            Problem, SimpleProblemType, ValidationProblem, VALIDATION_PROBLEM_MISSING_FIELD,
        },
        Response,
    },
    users::{Email, Password, UserData, Username},
};
use actix_http::http::StatusCode;
use actix_web::web::{Data, Json};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};

/// Representation of the input data to consume
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// The username submitted
    username: Option<String>,
    /// The email address submitted
    email: Option<String>,
    /// The display namesubmitted
    display_name: Option<String>,
    /// The password submitted
    password: Option<String>,
}

/// Problem to return for a duplicate username
const DUPLICATE_USERNAME: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:universe/2020:problems/authentication/duplicate_username",
    problem_title: "Duplicate Username",
    status_code: StatusCode::UNPROCESSABLE_ENTITY,
};

/// Problem to return for a duplicate email address
const DUPLICATE_EMAIL: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:universe/2020:problems/authentication/duplicate_email",
    problem_title: "Duplicate email address",
    status_code: StatusCode::UNPROCESSABLE_ENTITY,
};

/// HTTP handler to register a new user.
///
/// # Parameters
/// - `input` - The input data containing the user details
/// - `authentication_service` - The authentication service
///
/// # Returns
pub async fn register(
    input: Json<Input>,
    authentication_service: Data<Arc<dyn RegisterUserUseCase>>,
) -> Result<Response<HalPayload<AuthenticatedUserResponse>>, Problem> {
    let username = input
        .username
        .clone()
        .and_then(|username| Username::from_str(&username).ok())
        .ok_or_else(|| {
            ValidationProblem::default()
                .with_field("username", VALIDATION_PROBLEM_MISSING_FIELD)
                .build()
        })?;

    let email = input
        .email
        .clone()
        .and_then(|email| Email::from_str(&email).ok())
        .ok_or_else(|| {
            ValidationProblem::default()
                .with_field("email", VALIDATION_PROBLEM_MISSING_FIELD)
                .build()
        })?;

    let display_name = input.display_name.clone().ok_or_else(|| {
        ValidationProblem::default()
            .with_field("displayName", VALIDATION_PROBLEM_MISSING_FIELD)
            .build()
    })?;

    let password = input
        .password
        .clone()
        .map(Password::from_plaintext)
        .ok_or_else(|| {
            ValidationProblem::default()
                .with_field("password", VALIDATION_PROBLEM_MISSING_FIELD)
                .build()
        })?;

    let user_data = UserData {
        username,
        email,
        display_name,
        password,
    };

    let authenticated_user = authentication_service
        .register_user(user_data)
        .await
        .map_err(|e| {
            tracing::warn!(e = ?e, "User registration failure");
            match e {
                RegisterError::DuplicateEmail => Problem::from(DUPLICATE_EMAIL),
                RegisterError::DuplicateUsername => Problem::from(DUPLICATE_USERNAME),
            }
        })?;

    Ok(authenticated_user.into())
}
