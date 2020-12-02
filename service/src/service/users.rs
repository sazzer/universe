use std::sync::Arc;

use actix_web::web::ServiceConfig;

use crate::{
    database::Database,
    server::Configurer,
    users::{endpoints, service::UsersService, GetUserUseCase},
};

/// The Users Component.
pub struct UsersComponent {
    service: Arc<UsersService>,
}

impl Configurer for UsersComponent {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn GetUserUseCase>);

        endpoints::configure(config);
    }
}

/// Build the Users Component.
pub fn build(database: Arc<Database>) -> Arc<UsersComponent> {
    let service = Arc::new(UsersService::new(database));
    Arc::new(UsersComponent { service })
}
