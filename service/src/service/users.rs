use super::home::Contributor;
use crate::{
    database::Database,
    server::Configurer,
    users::{endpoints, service::UsersService, GetUserUseCase},
};
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// The Users Component.
pub struct Component {
    /// The users service.
    pub service: Arc<UsersService>,
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn GetUserUseCase>);

        endpoints::configure(config);
    }
}

impl Contributor for Component {}

/// Build the Users Component.
///
/// # Parameters
/// - `database` - The database connection
///
/// # Returns
/// The users component to wire in to other components.
pub fn build(database: Arc<Database>) -> Arc<Component> {
    let service = Arc::new(UsersService::new(database));
    Arc::new(Component { service })
}
