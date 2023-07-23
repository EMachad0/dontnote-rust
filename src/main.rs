fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();
 
    api::main();
}