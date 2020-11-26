#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::future_not_send)]

mod database;
mod server;
mod service;

pub use database::Config as DatabaseConfig;
pub use service::{testing::TestResponse, Config, Service};
