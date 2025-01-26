use sqlx::{FromRow, MySql, MySqlPool};
use mysql::{OptsBuilder, prelude::*};
use sqlx::types::time::Date;
use time::macros::format_description;
use num_bigint::BigInt;

use async_std::task;
use std::{collections::HashMap, u8};

use axum::{
    http::{header::{HeaderName, HeaderValue, CONTENT_TYPE, ACCEPT}, HeaderMap, StatusCode}, response::IntoResponse, routing::{get, post}, Error, Json, Router,
    extract::Query,
};

#[path = "obj_pageable.rs"] mod obj_pageable;
#[path = "obj_token.rs"] mod obj_token;
#[path = "obj_model.rs"] mod obj_model;
#[path = "database_connection.rs"] mod obj_database;

use jsonwebtoken::{encode, EncodingKey, Header, };

use obj_model::ToJson;
use serde::{Deserialize, Serialize};

//use actix_web::{web, App, HttpServer, Responder, HttpResponse, Error};
use actix_web::{web};
use serde_json::{from_str, Result, Value}; 


#[derive(Deserialize)]
pub struct FiltroUsuariosX {
    nome: Option<String>,
    nomeEntre: Option<String>,
    nomeInicia: Option<String>,
    nomeTermina: Option<String>,
}

#[derive(FromRow)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)] 
pub struct Usuariox {
    idUsuario: i64,
    nome: String,
}

///////////////////////////////////////////////////////////////////

pub async fn get_usuariox4(Query(filtro): Query<FiltroUsuariosX>, headers: HeaderMap) -> core::result::Result<Json<obj_pageable::Pageable<Usuariox>>, StatusCode> {
    
    // listando os valores do headers
    for (chave, valor) in headers {
        match chave {
            Some(name_param) => println!("O chave é: {}, o valor é: {}", name_param, valor.to_str().unwrap_or("Valor inválido")), // Imprime "O valor é: Olá, mundo!"
            None => println!("Não há valor."),
        }
    }

    let mut sql_select_base: String = "select idUsuario, IFNULL(nome, '') nome from usuario ".to_string();

    if filtro.nome.is_some() || filtro.nomeEntre.is_some()|| filtro.nomeInicia.is_some()|| filtro.nomeTermina.is_some() {
        sql_select_base.push_str(" WHERE ");

        match filtro.nome {
            Some(nome) => {
                sql_select_base.push_str(&format!("nome = '{}'", nome));
            },
            None => println!("Nome não informado"),
        }
        match filtro.nomeEntre {
            Some(nomeEntre) => {
                sql_select_base.push_str(&format!("nome like '%{}%'", nomeEntre));
            },
            None => println!("nomeEntre não informado"),
        }
        match filtro.nomeInicia {
            Some(nomeInicia) => {
                sql_select_base.push_str(&format!("nomeInicia like '{}%'", nomeInicia));
            },
            None => println!("nomeInicia não informado"),
        }
        match filtro.nomeTermina {
            Some(nomeTermina) => {
                sql_select_base.push_str(&format!("nome like '%{}'", nomeTermina));
            },
            None => println!("nomeTermina não informado"),
        }                        
    }

    let mut pageable : obj_pageable::Pageable<Usuariox> = obj_pageable::Pageable::new();

    let result = task::block_on(obj_database::connect());
    match result {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
        }        
        Ok(pool) => {
            let query_result: Vec<Usuariox> = sqlx::query_as::<_, Usuariox>(sql_select_base.as_str())
                .fetch_all(&pool).await.unwrap();

            println!("Number of Usuariox selected: {}", query_result.len());
            for (rindex, usuariox) in query_result.iter().enumerate() {
                /*
                pageable.adicionar_content(Usuariox{
                    idUsuario : usuariox.idUsuario,
                    nome : usuariox.nome.to_string()
                });
                */
                pageable.adicionar_content( usuariox.clone());
            }
            pageable.numberOfElements = pageable.content.len().try_into().unwrap();
            pageable.pageSize = obj_pageable::itens_por_pagina();            
            
        }        
    }        

    println!("Json(Usuario) ");
    Ok(Json(pageable))

}
