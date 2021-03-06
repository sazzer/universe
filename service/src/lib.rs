#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::future_not_send, clippy::module_name_repetitions)]

mod authentication;
mod authorization;
mod database;
mod health;
mod home;
mod http;
mod model;
mod server;
mod service;
mod users;

pub use database::Config as DatabaseConfig;
pub use service::{testing::TestResponse, Config, Service};
