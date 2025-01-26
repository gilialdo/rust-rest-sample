
use std::{collections::HashMap, u8};

use axum::{
    extract::{FromRequest, FromRequestParts, Path, Query, Request, State  }, handler, 
    http::{header::{HeaderName, HeaderValue, CONTENT_TYPE}, HeaderMap, StatusCode}, 
    middleware::Next, response::{IntoResponse, Response}, 
    routing::{get, post}, Json, Router
};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error};

use crate::obj_token;
use crate::obj_access_token;
use crate::obj_token_req;


pub async fn testeok2(Json(payload):Json<obj_token_req::TokenReqParams>) -> impl IntoResponse {

    let access_token = obj_access_token::AccessToken {
        sub: "b08a1717-a33b-4607-8c4d-2100c530c7f9".to_string(),
        exp: 10000000000,
        database: "".to_string(),
        authorities: vec![ 
            "ROLE_USUARIO_LISTAR".to_string(),
            "ROLE_USUARIO_CRIAR".to_string()
        ]
    };

    // Codifica o token JWT
    let key = EncodingKey::from_secret("secret".as_ref());

    let str_token = match encode(&Header::default(), &access_token, &key) {
        Ok(t) => t,
        Err(err) => return Err(StatusCode::INTERNAL_SERVER_ERROR), 
    };

    let token: obj_token::Token = obj_token::Token {
        access_token: str_token,
        token_type: String::from("bearer"), 
        expires_in: 1800, 
        scope:String::from("read write"),
        index_cliente: 0
    };

    Ok(Json(token))
}

async fn testeok(payload:String) -> impl IntoResponse {

    println!("header {}", payload);

    (StatusCode::CREATED, "")
}



async fn root() -> &'static str {
    "Hello, From protected Route"
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}





// Handler para GET /users
async fn get_users() -> Result<Json<Vec<User>>, StatusCode> {
    
    //println!("authorisation_header {}", HeaderValue::len(&self));
    // Aqui você normalmente buscaria usuários de um banco de dados
    let users = vec![
        User {
            id: 1,
            username: "user1".to_string(),
        },
        User {
            id: 2,
            username: "user2".to_string(),
        },
    ];
    println!("Json(users) ");
    Ok(Json(users))
}

// Handler para POST /users
async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    // Aqui você normalmente salvaria o usuário em um banco de dados
    let user = User {
        id: 1,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}


// Middleware para gerar token JWT (apenas para demonstração)
async fn generate_token(form: web::Form<HashMap<String, String>>) -> Result<Json<obj_token::Token>, StatusCode> {
    let access_token = obj_access_token::AccessToken {
        sub: "b08a1717-a33b-4607-8c4d-2100c530c7f9".to_string(),
        exp: 10000000000,
        database: "".to_string(),
        authorities: vec![ 
            "ROLE_USUARIO_LISTAR".to_string(),
            "ROLE_USUARIO_CRIAR".to_string()
        ]
    };

    // Codifica o token JWT
    let key = EncodingKey::from_secret("secret".as_ref());

    let str_token = match encode(&Header::default(), &access_token, &key) {
        Ok(t) => t,
        Err(err) => return Err(StatusCode::INTERNAL_SERVER_ERROR), 
    };

    let token: obj_token::Token = obj_token::Token {
        access_token: str_token,
        token_type: String::from("bearer"), 
        expires_in: 1800, 
        scope:String::from("read write"),
        index_cliente: 0
    };

    Ok(Json(token))
}
