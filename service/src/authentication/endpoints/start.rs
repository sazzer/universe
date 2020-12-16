use crate::{
    http::siren::Link,
    http::siren::SirenResponse,
    http::siren::{Action, Field, SirenPayload},
    http::Response,
    users::GetUserUseCase,
    users::Username,
};
use actix_web::web::{Data, Form};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};

/// Representation of the form data to consume
#[derive(Deserialize)]
pub struct FormData {
    /// The username submitted
    username: String,
}

/// HTTP handler to start authentication for a username.
///
/// # Parameters
/// - `form` - The form data containing the username
/// - `users_service` - The users service, to see if the username exists
///
/// # Returns
/// The Siren model for how to start authentication.
/// If the provided username is known to the system then returns the model for the Authentiate action.
/// If the provided username isn't known to the system then returns the model for the Register action.
pub async fn start(
    form: Form<FormData>,
    users_service: Data<Arc<dyn GetUserUseCase>>,
) -> Response<SirenPayload<()>> {
    let username = Username::from_str(&form.username).unwrap();
    let user = users_service.get_user_by_username(&username).await;

    let mut payload = SirenPayload::new(())
        .with_class("authentication")
        .with_link(Link::new("/authentication").with_rel("self"));

    if user.is_some() {
        payload = payload.with_action(
            Action::new("authenticate", "POST", "/authentication/authenticate")
                .with_type_form()
                .with_field(Field::new("username", "hidden").with_value(form.username.clone()))
                .with_field(Field::new("password", "password").with_class("enter-password")),
        );
    } else {
        payload = payload.with_action(
            Action::new("register", "POST", "/authentication/register")
                .with_type_form()
                .with_field(Field::new("username", "hidden").with_value(form.username.clone()))
                .with_field(Field::new("email", "email"))
                .with_field(Field::new("display_name", "text"))
                .with_field(Field::new("password", "password").with_class("set-password")),
        );
    }

    SirenResponse {
        body: Some(payload),
        ..SirenResponse::default()
    }
    .into()
}