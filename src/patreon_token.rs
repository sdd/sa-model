use chrono::{DateTime, Utc};

use dynomite::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Item, Eq, PartialEq, Serialize)]
pub struct PatreonToken {
    pub access_token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    #[dynomite(partition_key)]
    pub id: String, // sa user id
    pub patreon_id: String, // patreon user id
    pub refresh_token: String,
    pub scope: String,
    #[dynomite(sort_key)]
    pub updated_at: DateTime<Utc>,
}
