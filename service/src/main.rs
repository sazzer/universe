use dotenv::dotenv;

fn main() {
    dotenv().ok();

    universe_lib::main()
}
