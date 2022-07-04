use actix_web::{get, /* http::header, post, web, */App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
// use r2d2::Pool;
// use r2d2_sqlite::SqliteConnectionManager;
// use rusqlite::params;
// use serde::Deserialize;
use thiserror::Error;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "another.html")]
struct AnotherTemplate {
    data: i32
}

#[derive(Error, Debug)]
enum MyError{
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
    
    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError{}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let html = IndexTemplate {};
    let response_body = html.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}
#[get("/another")]
async fn another() -> Result<HttpResponse, MyError> {
    let data = 20020209;
    
    let html = AnotherTemplate {data};
    let response_body = html.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || {
        App::new()
        .service(index)
        .service(another)
    }).bind("0.0.0.0:8080")?.run().await?;
    
    Ok(())
}
