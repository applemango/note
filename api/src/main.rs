#![allow(legacy_derive_helpers)]
use actix_web::{get, post, web, App, HttpServer, Responder, Result, HttpResponse, HttpRequest};
use rusqlite::Connection;

mod structs;
use structs::MyError;

mod token;
use token::{
    isLogin,
    create_token,
    create_user,
    refresh_token,
    logout_user
};

mod note;
use note::{
    create_note,
    get_note,
    update_note,
    delete_note
};

#[get("/")]
async fn hello(req: HttpRequest) -> Result<impl Responder, MyError> {
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    println!("{:#?}", token_data);
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    {
        let conn = Connection::open("app.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id       INTEGER PRIMARY KEY AUTOINCREMENT,
                username STRING UNIQUE NOT NULL,
                password STRING NOT NULL
            )",
            ()
        ).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tokenblocklist (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                user  INTEGER,
                token STRING,
                uuid  STRING,
                exp   INTEGER
            )",
            ()
        ).unwrap();
    
        conn.execute(
            "CREATE TABLE IF NOT EXISTS note (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id     INTEGER,
                title       STRING,
                description STRING,
                body        STRING,
                FOREIGN     KEY(user_id) REFERENCES user (id)
            )",
            ()
        ).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tag (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER,
                user_id INTEGER,
                name    STRING,
                color   STRING,
                FOREIGN KEY(user_id) REFERENCES user (id),
                FOREIGN KEY(note_id) REFERENCES note (id)
            )",
            ()
        ).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS status (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER,
                user_id INTEGER,
                name    STRING,
                FOREIGN KEY(user_id) REFERENCES user (id),
                FOREIGN KEY(note_id) REFERENCES note (id)
            )",
            ()
        ).unwrap();
    }
    HttpServer::new(|| {
        App::new()
            .route("/user", web::post().to(create_user))
            .route("/token", web::post().to(create_token))
            .route("/token/refresh", web::get().to(refresh_token))
            .route("/token/logout", web::get().to(logout_user))
            
            .route("/note", web::get().to(get_note))
            .route("/note", web::post().to(create_note))
            .route("/note/{id}", web::delete().to(delete_note))
            .route("/note/{id}", web::post().to(update_note))
            .route("/note/tag", web::get().to(manual_hello))
            .route("/note/tag", web::post().to(manual_hello))
            .route("/note/tag/{id}", web::delete().to(manual_hello))
            .route("/note/{id}/tag/{tag_id}", web::post().to(manual_hello))
            .route("/note/{id}/tag/{tag_id}", web::delete().to(manual_hello))
            .route("/note/status", web::get().to(manual_hello))
            .route("/note/status", web::post().to(manual_hello))
            .route("/note/status/{id}", web::delete().to(manual_hello))
            .route("/note/{id}/status/{status_id}", web::post().to(manual_hello))
            .route("/note/{id}/status/{status_id}", web::delete().to(manual_hello))
            
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}