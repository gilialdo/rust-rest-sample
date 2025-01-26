use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenReqParams {

    pub username: String,
    pub password:  String,
    pub grant_type: String,  

}