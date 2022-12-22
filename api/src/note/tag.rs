use actix_web::{web, Responder, Result, HttpResponse, HttpRequest};
use rusqlite::Connection;
use serde::Deserialize;

pub use crate::structs::{MyError, Note, Tag, Status};
pub use crate::token::isLogin;

#[derive(Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: String,
}

pub async fn get_tag(req: HttpRequest) -> Result<impl Responder, MyError> {
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
            name,
            color
        FROM tag WHERE user_id = ?1") {
        Ok(a) => a,
        Err(_) => return Err(MyError {name: "not found"})
    };
    let all_item = match stmt.query_map([token_data.id], |row| {
        Ok(Tag {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            color: row.get(3)?
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

pub async fn create_tag(req: HttpRequest, body: web::Json<CreateTagRequest>) -> Result<impl Responder, MyError> {
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let mut statement = match db_con.prepare("INSERT INTO tag ( user_id, name, color ) values ( ?1, ?2, ?3 )") {
        Ok(statement) => statement,
        Err(_) => return Err(MyError {name: "db statement error"})
    };
    let _ = match statement.execute(&[&token_data.id, &body.name, &body.color]) {
        Ok(result) => result,
        Err(_) => return Err(MyError {name: "db execute error"})
    };

    let result = match db_con.query_row("
        SELECT
            id,
            user_id,
            name,
            color
        FROM tag WHERE id = last_insert_rowid()", [], |row| {
        Ok(Tag {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            color: row.get(3)?
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    
    Ok(web::Json(result))
}

pub async fn delete_tag(path: web::Path<i32>, req: HttpRequest) -> Result<impl Responder, MyError> {
    let _ = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let _ = match db_con.execute("DELETE tag WHERE id = ?1", [&path.to_string()]) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    Ok(HttpResponse::Ok().json("deleted"))
}

pub async fn get_note_tag(path: web::Path<i32>, req: HttpRequest) -> Result<impl Responder, MyError> {
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
            name,
            color
        FROM tag WHERE user_id = ?1, note_id = ?1") {
        Ok(a) => a,
        Err(_) => return Err(MyError {name: "not found"})
    };
    let all_item = match stmt.query_map([token_data.id, path.to_string()], |row| {
        Ok(Tag {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            color: row.get(3)?
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

pub async fn add_tag(req: HttpRequest) -> Result<impl Responder, MyError> {
    let (id, tag_id): (i32, i32) = req.match_info().load().unwrap();
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let mut statement = match db_con.prepare("INSERT INTO note_tag ( user_id, note_id, tag_id ) values ( ?1, ?2, ?3 )") {
        Ok(statement) => statement,
        Err(_) => return Err(MyError {name: "db statement error"})
    };
    let _ = match statement.execute(&[&token_data.id, &id.to_string(), &tag_id.to_string()]) {
        Ok(result) => result,
        Err(_) => return Err(MyError {name: "db execute error"})
    };

    let result = match db_con.query_row("
        SELECT
            id,
            user_id,
            name,
            color
        FROM tag WHERE id = last_insert_rowid()", [], |row| {
        Ok(Tag {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            color: row.get(3)?
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    
    Ok(web::Json(result))
}

pub async fn remove_tag(req: HttpRequest) -> Result<impl Responder, MyError> {
    let (id, tag_id): (i32, i32) = req.match_info().load().unwrap();
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let _ = match db_con.execute("DELETE note WHERE user_id = ?1, note_id = ?2, tag_id = ?3", &[&token_data.id, &id.to_string(), &tag_id.to_string()]) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    Ok(HttpResponse::Ok().json("deleted"))
}
