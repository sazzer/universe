#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use dotenv::dotenv;

/// Main entrypoint for the Universe application.
fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    universe_lib::main()
}
