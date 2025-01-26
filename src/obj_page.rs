
//mod obj_pageable;
//mod obj_sort;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Page {
	totalElements : i32,
	totalPages : i32,
	last : bool,
	size : i32,
	number : i32,
	sort: obj_sort::Sort,
	numberOfElements : i32,
	first : bool,
	empty : bool,
	pageable : obj_pageable::Pageable
}
