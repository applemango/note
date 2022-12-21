use actix_web::{
    web, Responder, Result, HttpResponse, HttpRequest
};
use jwt_simple::prelude::*;
use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use uuid::Uuid;



pub use crate::structs::{MyError, User, TokenClaims, TokenData};

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct CreateRefreshTokenResponse {
    pub token: String,
}

pub async fn create_user(data: web::Json<CreateTokenRequest>) -> Result<impl Responder, MyError> {
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(MyError {name: "db connection error"})
        }
    };

    let mut statement = match db_con.prepare("INSERT INTO user ( username, password ) values ( ?1, ?2 )") {
        Ok(statement) => statement,
        Err(_) => return Err(MyError {name: "Failed to prepare query".into()}),
    };

    let mut _r = match statement.execute(&[&data.username, &data.password]) {
        Ok(r) => r,
        Err(_) => return Err(MyError {name: "Failed"})
    };

    Ok(HttpResponse::Ok().body("Created"))
}

pub async fn create_token(data: web::Json<CreateTokenRequest>) -> Result<impl Responder, MyError> {
    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(MyError {name: "db connection error"})
        }
    };

    let u = match db_con.query_row("SELECT id, username, password FROM user WHERE username = ( ?1 )", [&data.username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?
        })
    }) {
        Ok(u) => u,
        Err(_) => {
            return Err(MyError {name: "not found"})
        }
    };

    if !(data.username == u.username && data.password == u.password) {
        return Err(MyError {name: "Invalid username or password"})
    }
    
    let token_key = HS256Key::from_bytes(b"secret");

    let token_claims = TokenClaims {
        refresh: false,
    };
    let claims = Claims::with_custom_claims(token_claims, Duration::from_mins(15)).with_subject(u.id).with_jwt_id(Uuid::new_v4().to_string());
    let token =  match token_key.authenticate(claims) {
        Ok(token) => token,
        Err(_) => return Err(MyError {name : "Invalid token"}),
    };

    let token_claims = TokenClaims {
        refresh: true,
    };
    let claims_ = Claims::with_custom_claims(token_claims, Duration::from_hours(24)).with_subject(u.id).with_jwt_id(Uuid::new_v4().to_string());
    let refresh_token_ =  match token_key.authenticate(claims_) {
        Ok(token) => token,
        Err(_) => return Err(MyError {name : "Invalid token"}),
    };
    
    Ok(web::Json(CreateTokenResponse {
        token: token,
        refresh_token: refresh_token_,
    }))
}

pub async fn refresh_token(req: HttpRequest) -> Result<impl Responder, MyError> {
    fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
        req.headers().get("Authorization")?.to_str().ok()
    }
    let token;
    if let Some(t) = get_content_type(&req) {
        token = &t[7..];
    } else {
        return Err(MyError {name : "Invalid token"})
    }

    let token_key = HS256Key::from_bytes(b"secret");
    let claims = match token_key.verify_token::<TokenClaims>(&token, None) {
        Ok(claims) => claims,
        Err(_) => return Err(MyError {name : "Invalid token"}),
    };

    if ! claims.custom.refresh {
        return Err(MyError {name : "Invalid token"})
    }

    let token_claims = TokenClaims {
        refresh: false,
    };

    let sub;
    match claims.subject {
        Some(d) => sub = d,
        None => return Err(MyError {name : "Invalid token"})
    }

    let claims = Claims::with_custom_claims(token_claims, Duration::from_mins(15)).with_subject(sub).with_jwt_id(Uuid::new_v4().to_string());
    let token =  match token_key.authenticate(claims) {
        Ok(token) => token,
        Err(_) => return Err(MyError {name : "Invalid token"}),
    };

    Ok(web::Json(CreateRefreshTokenResponse {
        token: token
    }))
}

pub async fn logout_user(req: HttpRequest) -> Result<impl Responder, MyError> {
    fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
        req.headers().get("Authorization")?.to_str().ok()
    }
    let token;
    if let Some(t) = get_content_type(&req) {
        token = &t[7..];
    } else {
        return Err(MyError {name : "Invalid token"})
    }

    let token_key = HS256Key::from_bytes(b"secret");

    let claims = match token_key.verify_token::<TokenClaims>(&token, None) {
        Ok(claims) => claims,
        Err(_) => return Err(MyError {name : "Invalid token"}),
    };

    let sub;
    let uuid;
    let exp;
    match claims.subject {
        Some(d) => sub = d,
        None => return Err(MyError {name : "Invalid token"})
    }
    match claims.jwt_id {
        Some(d) => uuid = d,
        None => return Err(MyError {name : "Invalid token"})
    }
    match claims.expires_at {
        Some(d) => exp = d.as_millis(),
        None => return Err(MyError {name : "Invalid token"})
    }

    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(MyError {name: "db connection error"})
        }
    };
    let mut statement = match db_con.prepare("INSERT INTO tokenblocklist ( user, token, uuid, exp ) values ( ?1, ?2, ?3, ?4 )") {
        Ok(statement) => statement,
        Err(_) => return Err(MyError {name: "Failed to prepare query".into()}),
    };

    let mut _r = match statement.execute((&sub, token, &uuid, &exp)) {
        Ok(r) => r,
        Err(_) => return Err(MyError {name: "Failed"})
    };

    Ok(HttpResponse::Ok().body("Deleted"))
}

pub async fn isLogin(req: HttpRequest) -> Result<TokenData, MyError> {
    fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
        req.headers().get("Authorization")?.to_str().ok()
    }
    let token;
    if let Some(t) = get_content_type(&req) {
        token = &t[7..];
    } else {
        return Err(MyError {name : "Invalid token"})
    }

    let token_key = HS256Key::from_bytes(b"secret");

    let claims = match token_key.verify_token::<TokenClaims>(&token, None) {
        Ok(claims) => claims,
        Err(_) => return Err(MyError {name : "Invalid token"}),
    };

    let db_con = match Connection::open("app.db") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(MyError {name: "db connection error"})
        }
    };

    let uuid;
    match claims.jwt_id {
        Some(d) => uuid = d,
        None => return Err(MyError {name : "Invalid token"})
    }

    let sub;
    let exp;
    match claims.subject {
        Some(d) => sub = d.parse::<i32>().unwrap(),
        None => return Err(MyError {name : "Invalid token"})
    }
    match claims.expires_at {
        Some(d) => exp = d.as_millis(),
        None => return Err(MyError {name : "Invalid token"})
    }

    let u = match db_con.query_row("SELECT uuid FROM tokenblocklist WHERE uuid = ( ?1 )", [&uuid], |_| {
        Ok(true)
    }) {
        Ok(_) => {
            return Err(MyError {name : "Invalid token"})
        },
        Err(_) => true
    };

    if !u {
        return Err(MyError {name : "Invalid token"})
    }

    if claims.custom.refresh {
        return Err(MyError {name : "Invalid token"})
    }
    let token_data = TokenData {
        id: uuid,
        sub: sub,
        exp: exp,
        TokenClaims: TokenClaims {
            refresh: claims.custom.refresh,
        }
    };
    Ok(token_data)
}

/* what is middleware?
pub struct JwtRequired;

impl<S, B> Transform<S, ServiceRequest> for JwtRequired
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    //type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtRequiredMiddleware<S, B>;
    type Future = future::FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(JwtRequiredMiddleware { service })
    }
}

struct JwtRequiredMiddleware<S, B>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    service: S,
}


impl<S, B> Service<ServiceRequest> for JwtRequiredMiddleware<S, B>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    //type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Result<futures::Async<()>, Self::Error> {
        Ok(futures::Async::Ready(()))
    }

    fn call(& self, req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();
        Box::new(self.service.call(ServiceRequest).map(|mut res| {
            fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
                req.headers().get("Authorization")?.to_str().ok()
            }
            let token;
            if let Some(t) = get_content_type(&req) {
                token = &t[7..];
            } else {
                return Err(MyError {name : "Invalid token"})
            }

            return Ok(svc.call(req).await?)
        }))
    }
}
*/