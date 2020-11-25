#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

mod database;
mod server;
mod service;

pub use database::Config as DatabaseConfig;
pub use service::{Config, Service};
