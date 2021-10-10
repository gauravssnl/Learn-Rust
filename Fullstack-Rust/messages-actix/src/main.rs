use messages_actix::MessageApp;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "atcitx-web=info");
    env_logger::init();
    let (host, port) = ("0.0.0.0".to_string(), 8080);
    let app = MessageApp::new(host, port);
    app.run()
}
