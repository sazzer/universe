#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::future_not_send, clippy::module_name_repetitions)]

mod database;
mod health;
mod server;
mod service;

pub use database::Config as DatabaseConfig;
pub use service::{testing::TestResponse, Config, Service};
