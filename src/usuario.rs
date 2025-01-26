use std::{collections::HashMap, u8};

use axum::{
//    extract::{FromRequest, FromRequestParts, Path, Query, Request, State  }, handler, 
    http::{header::{HeaderName, HeaderValue, CONTENT_TYPE}, HeaderMap, StatusCode}, 
    response::{IntoResponse}, 
    routing::{get, post}, Json, Router
};

//use axum::{
//    extract::{FromRequest, FromRequestParts, Path, Query, Request, State  }, handler, 
//    http::{header::{HeaderName, HeaderValue, CONTENT_TYPE}, HeaderMap, StatusCode}, 
//    middleware::Next, response::{IntoResponse, Response}, 
//    routing::{get, post}, Json, Router
//};

//use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use jsonwebtoken::{encode, EncodingKey, Header, };
use serde::{Deserialize, Serialize};

//use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error};
use actix_web::{web};


#[derive(Serialize, Deserialize)]
pub struct Usuario {
    id: u64,
    nome: String,
}


// Handler para GET /users
pub async fn get_usuario() -> Result<Json<Vec<Usuario>>, StatusCode> {
    
    //println!("authorisation_header {}", HeaderValue::len(&self));
    // Aqui você normalmente buscaria usuários de um banco de dados
    let users = vec![
        Usuario {
            id: 1,
            nome: "Usuario1".to_string(),
        },
        Usuario {
            id: 2,
            nome: "Usuario2".to_string(),
        },
    ];
    println!("Json(Usuario) ");
    Ok(Json(users))

}
