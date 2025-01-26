use std::{collections::HashMap, u8};

use axum::{
    extract::{FromRequest, FromRequestParts, Path, Query, Request, State  }, handler, 
    http::{header::{HeaderName, HeaderValue, CONTENT_TYPE}, HeaderMap, StatusCode}, 
    middleware::Next, response::{IntoResponse, Response}, 
    routing::{get, post}, Json, Router
};

use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};


#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AccessToken {

    pub sub: String,
    pub exp: u64,
    pub database: String,  

    pub authorities: Vec<String>,

}

