#![allow(legacy_derive_helpers)]
use actix_web::{get, post, web, App, HttpServer, Responder, Result, HttpResponse, HttpRequest};
use rusqlite::Connection;
use actix_cors::Cors;

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
use note::note:: {
    create_note,
    get_note,
    update_note,
    delete_note
};
use note::tag:: {
    create_tag,
    delete_tag,
    get_tag,
    add_tag,
    remove_tag,
    get_note_tag
};
use note::status:: {
    create_status,
    delete_status,
    get_status,
    add_status,
    //remove_status,
    //get_note_status
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
            "CREATE TABLE IF NOT EXISTS status (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                name    STRING,
                FOREIGN KEY(user_id) REFERENCES user (id)
            )",
            ()
        ).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tag (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                name    STRING,
                color   STRING,
                FOREIGN KEY(user_id) REFERENCES user (id)
            )",
            ()
        ).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS note (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id     INTEGER,
                status_id   INTEGER,
                title       STRING,
                description STRING,
                body        STRING,
                FOREIGN     KEY(user_id) REFERENCES user (id),
                FOREIGN     KEY(status_id) REFERENCES status (id)
            )",
            ()
        ).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS note_tag (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                note_id INTEGER,
                tag_id INTEGER,
                FOREIGN KEY(user_id) REFERENCES user (id),
                FOREIGN KEY(note_id) REFERENCES note (id),
                FOREIGN KEY(tag_id) REFERENCES tag (id)
            )",
            ()
        ).unwrap();
    }
    HttpServer::new(|| {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            //.allowed_methods(vec!["GET", "POST", "DELETE"])
            //.allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            //.allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .route("/user", web::post().to(create_user))
            .route("/token", web::post().to(create_token))
            .route("/token/refresh", web::get().to(refresh_token))
            .route("/token/logout", web::get().to(logout_user))
            
            .route("/note", web::get().to(get_note))
            .route("/note", web::post().to(create_note))
            .route("/note/{id}", web::delete().to(delete_note))
            .route("/note/{id}", web::post().to(update_note))

            .route("/note/tag", web::get().to(get_tag))
            /*
                When I set path to /note/tag, I get an error "can not parse "tag" to a i32", which I think is due to a conflict with another route.
                Therefore, I changed the path to /note/tag/create this time.
            */
            .route("/note/tag/create", web::post().to(create_tag))
            .route("/note/tag/{id}", web::delete().to(delete_tag))
            .route("/note/{id}/tag", web::get().to(get_note_tag))
            .route("/note/{id}/tag/{tag_id}", web::post().to(add_tag))
            .route("/note/{id}/tag/{tag_id}", web::delete().to(remove_tag))
            
            .route("/note/status", web::get().to(get_status))
            .route("/note/status/create", web::post().to(create_status))
            .route("/note/status/{id}", web::delete().to(delete_status))
            //.route("/note/{id}/status", web::get().to(delete_status))
            .route("/note/{id}/status/{status_id}", web::post().to(add_status))
            //.route("/note/{id}/status/{status_id}", web::delete().to(remove_status))
            
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}