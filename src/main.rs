use actix_web::{get, /*http::header, post,*/ web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
// use serde::Deserialize;
use thiserror::Error;
// mod tables;

struct EventEntry {
    id: u32,
    name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<EventEntry>
}

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
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT * FROM events")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let name = row.get(1)?;
        Ok(EventEntry{id, name})
    })?;

    //データ反映用
    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }
    if entries.len() <= 0 {
        conn.execute("INSERT INTO events (name) VALUES (?)", ["学習塾"])?;
        conn.execute("INSERT INTO events (name) VALUES (?)", ["料理教室"])?;
        conn.execute("INSERT INTO events (name) VALUES (?)", ["ヨガ教室"])?;
        conn.execute("INSERT INTO events (name) VALUES (?)", ["プログラミング教室"])?;
    }

    let html = IndexTemplate {entries};
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
    let manager = SqliteConnectionManager::file("reserve.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool.");
    let conn = pool.get().expect("Failed to get the connection from the pool.");
    
    //イベントテーブル
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
        params![],
    ).expect("Failed to create a table `events`.");

    HttpServer::new(move || {
        App::new()
        .service(index)
        .service(another)
        .data(pool.clone())
    }).bind("0.0.0.0:8080")?.run().await?;
    
    Ok(())
}
