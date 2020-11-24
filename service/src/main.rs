#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    universe_lib::main()
}
