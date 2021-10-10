#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;

pub struct MessageApp {
    host: String,
    port: u16,
}

impl MessageApp {
    pub fn new(host: String, port: u16) -> Self {
        MessageApp { host, port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: {}:{}", self.host, self.port);
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
        .bind((self.host.clone(), self.port))?
        .workers(8)
        .run()
    }
}

#[derive(Serialize)]
struct IndexRespone {
    message: String,
}

#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexRespone>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");
    Ok(web::Json(IndexRespone {
        message: hello.to_owned(),
    }))
}
