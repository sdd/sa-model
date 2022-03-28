use dynomite::Item;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action", content = "data")]
#[non_exhaustive]
pub enum WsMessage {
    PlateSolveQuery(),
    PlateSolveResult(),
    Heartbeat(),
}

#[derive(Clone, Debug, Deserialize, Item, PartialEq, Serialize)]
pub struct Connection {
    #[dynomite(partition_key)]
    pub id: String,
    pub ttl_expire_at: i64,
}
