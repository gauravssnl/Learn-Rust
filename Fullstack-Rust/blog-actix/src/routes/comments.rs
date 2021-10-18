use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
struct CommentInput {
    user_id: i32,
    body: String,
}

fn add_comment(
    post_id: web::Path<i32>,
    comments: web::Json<CommentInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let data = comments.into_inner();
        let user_id = data.user_id;
        let body = data.body;
        models::create_comment(conn, user_id, post_id.into_inner(), body.as_str())
    })
    .then(convert)
}

fn get_post_comments(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let post_id = post_id.into_inner();
        models::get_post_comments(conn, post_id)
    })
    .then(convert)
}

fn user_comments(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let user_id = user_id.into_inner();
        models::user_comments(conn, user_id)
    })
    .then(convert)
}

fn all_posts_with_comments(
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        models::all_posts_with_comments(conn)
    })
    .then(convert)
}

fn user_posts_with_comments(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let user_id = user_id.into_inner();
        models::user_posts_with_comments(conn, user_id)
    })
    .then(convert)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users/{id}/comments").route(web::get().to_async(user_comments)))
        .service(
            web::resource("/posts/{id}/comments")
                .route(web::post().to_async(add_comment))
                .route(web::get().to_async(get_post_comments)),
        )
        .service(
            web::resource("/users/{id}/posts/comments")
                .route(web::get().to_async(user_posts_with_comments)),
        )
        .service(
            web::resource("/posts/comments").route(web::get().to_async(all_posts_with_comments)),
        );
}
