use dynomite::Item;
use serde::{Deserialize, Serialize};
use crate::patreon_status::PatronStatus;
use crate::role::Role;

#[derive(Debug, Deserialize, Item, PartialEq, Serialize)]
pub struct User {
    #[dynomite(partition_key)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub patreon_status: PatronStatus,
    pub patreon_connected: bool,
}
