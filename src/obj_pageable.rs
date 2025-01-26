use serde::{Deserialize, Serialize};


#[path = "database_connection.rs"] mod obj_database;
use async_std::task;

#[path = "obj_sort.rs"] mod obj_sort;

#[derive(Deserialize)]
pub struct PageableFilter {

	pub page : Option<i32>,
	pub numberOfElements : Option<i32>,
	
}



#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Pageable<T> {
	pub content: Vec<T>,

	pub sort : obj_sort::Sort,
	pub offset : i32,
	pub pageSize : i32,
	pub pageNumber : i32,
	pub paged : bool,
	pub unpaged : bool,

	pub numberOfElements : i32,
	pub first : bool,
	pub empty : bool,	
}

impl<T> Pageable <T> {
    pub fn new() -> Self {
        Pageable { 
			content: Vec::new(),
			sort : obj_sort::Sort{ 
				 sorted : false,
				 unsorted : false
			},
			offset : 0,
			pageSize : 0,
			pageNumber : 0,
			paged : true,
			unpaged : false,
		
			numberOfElements : 0,
			first : false,
			empty : true,	
		}
    }

    pub fn adicionar_content(&mut self, item: T) {
        self.content.push(item);
    }

    pub fn obter_content(&self) -> &Vec<T> {
        &self.content
    }
	
	

}


pub fn itens_por_pagina() -> i32{
	let quant : i32 = 20;
	return quant;
}
