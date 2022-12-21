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
    pub title: String,
    pub description: String,
    pub body: String
}