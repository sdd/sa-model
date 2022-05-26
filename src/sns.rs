use crate::solvastro::Query;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SnsSolveRequestMessage {
    pub id: String,
    pub user_id: String,
    pub connection_id: String,
    pub query: Query,
}
