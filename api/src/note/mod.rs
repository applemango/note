use actix_web::{web, Responder, Result, HttpResponse, HttpRequest};
use rusqlite::Connection;
use serde::{Serialize, Deserialize};

pub use crate::structs::{MyError, Note};
pub use crate::token::isLogin;

pub async fn get_note(req: HttpRequest) -> Result<impl Responder, MyError> {
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    /*
    let stmt = match db_con.query_row("SELECT * from note WHERE user_id = ?1",[token_data.sub], |row| {
        Ok(Note {
            id: row.get(0)?,
            user_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            body: row.get(4)?
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    */
    let mut stmt = match db_con.prepare("
        SELECT
            id,
            user_id,
            title,
            description,
            body
        FROM note WHERE user_id = ?1") {
        Ok(a) => a,
        Err(_) => return Err(MyError {name: "not found"})
    };
    let all_item = match stmt.query_map([token_data.id], |row| {
        Ok(Note {
            id: row.get(0)?,
            user_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            body: row.get(4)?
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

pub async fn create_note(req: HttpRequest) -> Result<impl Responder, MyError> {
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let mut statement = match db_con.prepare("INSERT INTO note ( user_id ) values (?1)") {
        Ok(statement) => statement,
        Err(_) => return Err(MyError {name: "db statement error"})
    };
    let _ = match statement.execute(&[&token_data.id]) {
        Ok(result) => result,
        Err(_) => return Err(MyError {name: "db execute error"})
    };

    let result = match db_con.query_row("
        SELECT
            id,
            user_id,
            title,
            description,
            body
        FROM note WHERE id = last_insert_rowid()", [], |row| {
        Ok(Note {
            id: row.get(0)?,
            user_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            body: row.get(4)?
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    
    Ok(web::Json(result))
}

#[derive(Deserialize)]
pub struct UpdateNoteRequest {
    pub title: String,
    pub description: String,
    pub body: String
}

pub async fn update_note(path: web::Path<i32>, req: HttpRequest, body: web::Json<UpdateNoteRequest>) -> Result<impl Responder, MyError> {
    let token_data = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };

    let _ = match db_con.execute("UPDATE note SET title = ?1, description = ?2, body = ?3 WHERE id = ?4, user_id = ?5", [
        &body.title, &body.description, &body.body, &path.to_string(), &token_data.sub.to_string()
    ]) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };

    let result = match db_con.query_row("
        SELECT
            id,
            user_id,
            title,
            description,
            body
        FROM note WHERE id = ?1", [&path.to_string()], |row| {
        Ok(Note {
            id: row.get(0)?,
            user_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            body: row.get(4)?
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };

    Ok(web::Json(result))
}

pub async fn delete_note(path: web::Path<i32>, req: HttpRequest) -> Result<impl Responder, MyError> {
    let _ = match isLogin(req).await {
        Ok(token) => token,
        Err(err) => return Err(err)
    };
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => return Err(MyError {name: "db connection error"})
    };
    let _ = match db_con.execute("DELETE note WHERE id = ?1", [&path.to_string()]) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    Ok(HttpResponse::Ok().json("deleted"))
}