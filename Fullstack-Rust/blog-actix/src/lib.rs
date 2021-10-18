#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod errors;
mod models;
mod routes;
mod schema;

pub struct Blog {
    host: String,
    port: u16,
}

impl Blog {
    pub fn new(host: String, port: u16) -> Self {
        Blog { host, port }
    }

    pub fn run(self, database_url: String) -> std::io::Result<()> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        println!("Statring http server: {}:{}", self.host, self.port);
        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .configure(routes::users::configure)
                .configure(routes::posts::configure)
                .configure(routes::comments::configure)
        })
        .bind((self.host, self.port))?
        .run()
    }
}
