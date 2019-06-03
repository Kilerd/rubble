use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}
