#![allow(legacy_derive_helpers)]
use actix_web::{get, post, web, App, HttpServer, Responder, Result, error, HttpResponse, HttpRequest};
use jwt_simple::prelude::*;
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use uuid::Uuid;

mod structs;
use structs::{User, MyError};

mod token;
use token::{
    isLogin,
    create_token,
    create_user,
    refresh_token,
    logout_user
};

#[get("/")]
async fn hello(req: HttpRequest) -> Result<impl Responder, MyError> {
    let TokenData = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(MyError {name: "Invalid Token"})
    };
    println!("{:#?}", TokenData);
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
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username STRING UNIQUE NOT NULL,
                password STRING NOT NULL
            )",
            ()
        ).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tokenblocklist (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user INTEGER,
                token STRING,
                uuid STRING,
                exp INTEGER
            )",
            ()
        ).unwrap();
    }
    HttpServer::new(|| {
        App::new()
            .route("/user", web::post().to(create_user  ))
            .route("/token", web::post().to(create_token ) )
            .route("/token/refresh", web::get().to(refresh_token))
            .route("/token/logout", web::get().to(logout_user  ))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}