use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    const LOCAL_HOST: &str = "localhost";
    const LOCAL_PORT: i32 = 3000;

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    let bind_address = format!("{}:{}", LOCAL_HOST, LOCAL_PORT);
    println!("Serving on http://{}", bind_address);
    server
        .bind(&bind_address)
        .expect(&format!(
            "Error binding server to address: {}",
            bind_address
        ))
        .run()
        .expect("Error in running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
    <title>GCD Claculator</title>
    <form action="/gcd" method="post">
    <input type="text" name="n"/>
    <input type="text" name="m"/>
    <button type="submit">Commpute GCD</button>
    "#,
    )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Please enter non-zero n & m values")
    } else {
        HttpResponse::Ok().content_type("text/html").body(format!(
            "The greatest common divisor of the numbers {} and {} is <b> {}",
            form.n,
            form.m,
            gcd::compute(form.n, form.m)
        ))
    }
}
