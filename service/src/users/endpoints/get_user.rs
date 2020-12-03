use super::model::UserResponse;
use crate::{
    http::problem::Problem,
    http::problem::NOT_FOUND,
    http::{HalResponse, Response},
    users::GetUserUseCase,
    users::UserID,
};
use actix_web::web::{Data, Path};
use std::sync::Arc;

pub async fn get_user(
    path: Path<String>,
    users_service: Data<Arc<dyn GetUserUseCase>>,
) -> Result<Response<HalResponse<UserResponse>>, Problem> {
    let user_id: UserID = path.0.parse().map_err(|e| {
        tracing::warn!(e = ?e, user_id = ?path.0, "Failed to parse User ID");
        Problem::new(NOT_FOUND)
    })?;

    let user = users_service.get_user(user_id).await.ok_or_else(|| {
        tracing::info!(user_id = ?path.0, "Failed to find user");
        Problem::new(NOT_FOUND)
    })?;

    Ok(user.into())
}
