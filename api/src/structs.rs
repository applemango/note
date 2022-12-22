#![allow(legacy_derive_helpers)]
use actix_web::error;
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

#[display(fmt = "my error: {}", name)]
#[derive(Debug, Display, Error)]
pub struct MyError {
    pub name: &'static str,
}
impl error::ResponseError for MyError {}


#[derive(Serialize)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct TokenClaims {
    pub refresh: bool,
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct TokenData {
    pub id: String,
    pub sub: i32,
    pub exp: u64,
    pub TokenClaims: TokenClaims,
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub user_id: i32,
    pub status_id: i32,
    pub title: String,
    pub description: String,
    pub body: String
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct NoteTag {
    pub id: i32,
    pub user_id: i32,
    pub note_id: i32,
    pub tag_id: i32,
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct Tag {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub color: String
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct Status {
    pub id: i32,
    pub user_id: i32,
    pub name: String
}