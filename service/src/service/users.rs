use std::sync::Arc;

use actix_web::web::ServiceConfig;

use crate::{
    server::Configurer,
    users::{service::UsersService, GetUserUseCase},
};

/// The Users Component.
pub struct UsersComponent {
    service: Arc<UsersService>,
}

impl Configurer for UsersComponent {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn GetUserUseCase>);
    }
}

/// Build the Users Component.
pub fn build() -> Arc<UsersComponent> {
    let service = Arc::new(UsersService::new());
    Arc::new(UsersComponent { service })
}
