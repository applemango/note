use actix_web::{web, Responder, Result, HttpResponse, HttpRequest};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub use crate::structs::{MyError, Note, Tag, Status};
pub use crate::token::isLogin;

#[derive(Deserialize)]
pub struct UpdateNoteRequest {
    pub title: String,
    pub description: String,
    pub body: String
}

#[derive(Serialize)]
pub struct NoteResponseTag {
    pub id: i32,
    pub name: String,
    pub color: String,
}
#[derive(Serialize)]
pub struct NoteResponse {
    pub id: i32,
    pub user_id: i32,
    pub status_id: i32,
    pub status_name: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Vec<NoteResponseTag>,
}


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
            status_id,
            title,
            description,
            body
        FROM note WHERE user_id = ?1") {
        Ok(a) => a,
        Err(_) => return Err(MyError {name: "not found"})
    };
    let all_item = match stmt.query_map([token_data.sub], |row| {
        Ok(Note {
            id: row.get(0)?,
            user_id: row.get(1)?,
            status_id: match row.get(2) {
                Ok(note) => note,
                Err(_) => -1
            },
            title: match row.get(3) {
                Ok(note) => note,
                Err(_) => "".to_string()
            },
            description: match row.get(4) {
                Ok(note) => note,
                Err(_) => "".to_string()
            },
            body: match row.get(5) {
                Ok(note) => note,
                Err(_) => "".to_string()
            }
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
            Ok(v) => {
                let status = match db_con.query_row("
                    SELECT
                        id,
                        user_id,
                        name
                    FROM status WHERE id = ?1", [v.status_id], |row| {
                    Ok(Status {
                        id: row.get(0)?,
                        user_id: row.get(1)?,
                        name: row.get(2)?
                    })
                }) {
                    Ok(u) => u,
                    Err(_) => {
                        Status {
                            id: -1,
                            user_id: v.user_id,
                            name: "None".to_string()
                        }
                    }
                };

                let mut stmt = match db_con.prepare("
                    SELECT * FROM tag INNER JOIN note_tag ON tag.id = note_tag.tag_id WHERE tag.user_id = ?1 AND note_id = ?2") {
                    Ok(a) => a,
                    Err(_) => return Err(MyError {name: "not found"})
                };
                let all_tag = match stmt.query_map([token_data.sub.to_string(), v.id.to_string()], |row| {
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
                let mut tags = Vec::new();
                for item in all_tag {
                    tags.push(match item {
                        Ok(v) => NoteResponseTag {
                            id: v.id,
                            name: v.name,
                            color: v.color
                        },
                        Err(_) => return Err(MyError {name: "error"})
                    })
                }

                NoteResponse {
                    id: v.id,
                    user_id: v.user_id,
                    title: v.title,
                    description: v.description,
                    body: v.body,
                    status_id: status.id,
                    status_name: status.name,
                    tags: tags
                }
            },
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
    let mut statement = match db_con.prepare("INSERT INTO note ( user_id ) values ( ?1 )") {
        Ok(statement) => statement,
        Err(_) => return Err(MyError {name: "db statement error"})
    };
    let mut _e = match statement.execute(&[&token_data.sub.to_string()]) {
        Ok(result) => result,
        Err(e) => {
            println!("{}",e);
            return Err(MyError {name: "db execute error"})
        }
    };

    let result = match db_con.query_row("
        SELECT
            id,
            user_id,
            status_id,
            title,
            description,
            body
        FROM note WHERE id = last_insert_rowid()", [], |row| {
        Ok(Note {
            id: row.get(0)?,
            user_id: row.get(1)?,
            status_id: match row.get(2) {
                Ok(note) => note,
                Err(_) => -1
            },
            title: match row.get(3) {
                Ok(note) => note,
                Err(_) => "".to_string()
            },
            description: match row.get(4) {
                Ok(note) => note,
                Err(_) => "".to_string()
            },
            body: match row.get(5) {
                Ok(note) => note,
                Err(_) => "".to_string()
            }
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    
    Ok(web::Json(result))
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

    let _ = match db_con.execute("UPDATE note SET title = ?1, description = ?2, body = ?3 WHERE id = ?4 AND user_id = ?5", [
        &body.title, &body.description, &body.body, &path.to_string(), &token_data.sub.to_string()
    ]) {
        Ok(u) => u,
        Err(e) => {
            print!("{}",e);
            return Err(MyError {name: "not found"})
        }
    };

    let result = match db_con.query_row("
        SELECT
            id,
            user_id,
            status_id,
            title,
            description,
            body
        FROM note WHERE id = ?1", [&path.to_string()], |row| {
        Ok(Note {
            id: row.get(0)?,
            user_id: row.get(1)?,
            status_id: match row.get(2) {
                Ok(note) => note,
                Err(_) => -1
            },
            title: match row.get(3) {
                Ok(note) => note,
                Err(_) => "".to_string()
            },
            description: match row.get(4) {
                Ok(note) => note,
                Err(_) => "".to_string()
            },
            body: match row.get(5) {
                Ok(note) => note,
                Err(_) => "".to_string()
            }
        })
    }) {
        Ok(u) => u,
        Err(e) => {
            println!("{}", e);
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
    let _ = match db_con.execute("DELETE FROM note WHERE id = ?1", [&path.to_string()]) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };
    Ok(HttpResponse::Ok().json("deleted"))
}