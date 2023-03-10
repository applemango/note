use actix_web::{web, Responder, Result, HttpResponse, HttpRequest};
use rusqlite::Connection;
use serde::Deserialize;

pub use crate::structs::{MyError, Note, Tag, Status};
pub use crate::token::isLogin;

#[derive(Deserialize)]
pub struct CreateStatusRequest {
    pub name: String,
}

pub async fn get_status(req: HttpRequest) -> Result<impl Responder, MyError> {
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let mut stmt = match db_con.prepare("
        SELECT
            id,
            user_id,
            name
        FROM status WHERE user_id = ?1") {
        Ok(a) => a,
        Err(_) => return Err(MyError {name: "not found"})
    };
    let all_item = match stmt.query_map([token_data.sub], |row| {
        Ok(Status {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    let mut all = Vec::new();
    for item in all_item {
        all.push(match item {
            Ok(v) => v,
            Err(_) => return Err(MyError {name: "error"})
        })
    }
    Ok(web::Json(all))
}

pub async fn create_status(req: HttpRequest, body: web::Json<CreateStatusRequest>) -> Result<impl Responder, MyError> {
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let mut statement = match db_con.prepare("INSERT INTO status ( user_id, name ) values ( ?1, ?2 )") {
        Ok(statement) => statement,
        Err(_) => return Err(MyError {name: "db statement error"})
    };
    let _ = match statement.execute(&[&token_data.sub.to_string(), &body.name]) {
        Ok(result) => result,
        Err(_) => return Err(MyError {name: "db execute error"})
    };

    let result = match db_con.query_row("
        SELECT
            id,
            user_id,
            name
        FROM status WHERE id = last_insert_rowid()", [], |row| {
        Ok(Status {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    
    Ok(web::Json(result))
}

pub async fn delete_status(path: web::Path<i32>, req: HttpRequest) -> Result<impl Responder, MyError> {
    let _ = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let _ = match db_con.execute("DELETE FROM status WHERE id = ?1", [&path.to_string()]) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    Ok(HttpResponse::Ok().json("deleted"))
}

/*pub async fn get_note_status(path: web::Path<i32>, req: HttpRequest) -> Result<impl Responder, MyError> {

}*/
pub async fn add_status(req: HttpRequest) -> Result<impl Responder, MyError> {
    let (id, status_id): (i32, i32) = req.match_info().load().unwrap();
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let _ = match db_con.execute("UPDATE note SET status_id = ?1 WHERE id = ?2 AND user_id = ?3", [
        status_id, id, token_data.sub
    ]) {
        Ok(u) => u,
        Err(e) => {
            print!("{}",e);
            return Err(MyError {name: "not found"})
        }
    };
    Ok(HttpResponse::Ok().json("changed"))
}

/*pub async fn remove_status(path: web::Path<i32>, req: HttpRequest) -> Result<impl Responder, MyError> {
    
}*/