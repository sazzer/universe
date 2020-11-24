use dotenv::dotenv;

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    universe_lib::main()
}
