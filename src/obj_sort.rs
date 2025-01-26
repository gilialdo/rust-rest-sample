use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Sort {
	
	pub sorted : bool,
	pub unsorted : bool
	
}