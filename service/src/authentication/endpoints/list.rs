use std::sync::Arc;

use actix_web::web::Data;

use crate::{
    authentication::ListProvidersUseCase,
    http::{HalResponse, Response},
};

pub async fn list_providers(
    authentication_service: Data<Arc<dyn ListProvidersUseCase>>,
) -> Response<HalResponse<()>> {
    let providers = authentication_service.list_providers();

    providers.into()
}
