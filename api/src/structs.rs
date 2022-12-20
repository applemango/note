#![allow(legacy_derive_helpers)]
use actix_web::{get, post, web, App, HttpServer, Responder, Result, error, HttpResponse, HttpRequest};
use jwt_simple::prelude::*;
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use uuid::Uuid;

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