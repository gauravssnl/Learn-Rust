use std::env;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (host, port) = ("0.0.0.0".to_string(), 8080);
    let app = blog_actix::Blog::new(host, port);
    app.run(database_url)
}
