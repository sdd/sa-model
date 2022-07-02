use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub pc: bool, // patreon connected
    pub ps: bool, // patreon supporter
    pub exp: usize,
}
