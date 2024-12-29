use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    // additional fields...
}