pub mod patreon_status;
pub mod patreon_token;
pub mod role;
pub mod sns;
pub mod solvastro;
pub mod user;
pub mod websocket;
pub mod claims;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use super::patreon_token::PatreonToken;
    use chrono::{TimeZone, Utc};
    use dynomite::{AttributeValue, Attributes};

    #[test]
    fn can_convert_patreon_token_to_ddb_attrs() {
        let token = PatreonToken {
            access_token: "TEST_ACCESS_TOKEN".to_string(),
            created_at: Utc.ymd(2022, 2, 6).and_hms(1, 20, 0),
            expires_at: Utc.ymd(2022, 3, 6).and_hms(1, 20, 0),
            id: "TEST_ID".to_string(),
            patreon_id: "TEST_PATREON_ID".to_string(),
            refresh_token: "TEST_REFRESH_TOKEN".to_string(),
            scope: "TEST_SCOPE".to_string(),
            updated_at: Utc.ymd(2022, 2, 7).and_hms(0, 1, 0),
        };

        let mut token_ddb: Attributes = token.into();

        assert_eq!(
            token_ddb.get("access_token").unwrap().as_s().unwrap(),
            "TEST_ACCESS_TOKEN"
        );
        assert_eq!(
            token_ddb.remove("created_at").unwrap().as_s().unwrap(),
            "2022-02-06T01:20:00+00:00"
        );
        assert_eq!(
            token_ddb.get("expires_at").unwrap().as_s().unwrap(),
            "2022-03-06T01:20:00+00:00"
        );
        assert_eq!(token_ddb.get("id").unwrap().as_s().unwrap(), "TEST_ID");
        assert_eq!(
            token_ddb.get("patreon_id").unwrap().as_s().unwrap(),
            "TEST_PATREON_ID"
        );
        assert_eq!(
            token_ddb.get("refresh_token").unwrap().as_s().unwrap(),
            "TEST_REFRESH_TOKEN"
        );
        assert_eq!(
            token_ddb.get("scope").unwrap().as_s().unwrap(),
            "TEST_SCOPE"
        );
        assert_eq!(
            token_ddb.get("updated_at").unwrap().as_s().unwrap(),
            "2022-02-07T00:01:00+00:00"
        );
    }

    #[test]
    fn can_convert_ddb_attrs_to_patreon_token() {
        let token = PatreonToken {
            access_token: "TEST_ACCESS_TOKEN".to_string(),
            created_at: Utc.ymd(2022, 2, 6).and_hms(1, 20, 0),
            expires_at: Utc.ymd(2022, 3, 6).and_hms(1, 20, 0),
            id: "TEST_ID".to_string(),
            patreon_id: "TEST_PATREON_ID".to_string(),
            refresh_token: "TEST_REFRESH_TOKEN".to_string(),
            scope: "TEST_SCOPE".to_string(),
            updated_at: Utc.ymd(2022, 2, 7).and_hms(0, 1, 0),
        };

        let token_ddb: Attributes = HashMap::from([
            (
                "access_token".to_string(),
                AttributeValue::S("TEST_ACCESS_TOKEN".to_string()),
            ),
            (
                "created_at".to_string(),
                AttributeValue::S("2022-02-06T01:20:00+00:00".to_string()),
            ),
            (
                "expires_at".to_string(),
                AttributeValue::S("2022-03-06T01:20:00+00:00".to_string()),
            ),
            ("id".to_string(), AttributeValue::S("TEST_ID".to_string())),
            (
                "patreon_id".to_string(),
                AttributeValue::S("TEST_PATREON_ID".to_string()),
            ),
            (
                "refresh_token".to_string(),
                AttributeValue::S("TEST_REFRESH_TOKEN".to_string()),
            ),
            (
                "scope".to_string(),
                AttributeValue::S("TEST_SCOPE".to_string()),
            ),
            (
                "updated_at".to_string(),
                AttributeValue::S("2022-02-07T00:01:00+00:00".to_string()),
            ),
        ]);

        let token_2: PatreonToken = PatreonToken::try_from(token_ddb).unwrap();

        assert_eq!(token_2, token);
    }

    #[test]
    fn can_convert_patreon_token_to_json() {
        let token = PatreonToken {
            access_token: "TEST_ACCESS_TOKEN".to_string(),
            created_at: Utc.ymd(2022, 2, 6).and_hms(1, 20, 0),
            expires_at: Utc.ymd(2022, 3, 6).and_hms(1, 20, 0),
            id: "TEST_ID".to_string(),
            patreon_id: "TEST_PATREON_ID".to_string(),
            refresh_token: "TEST_REFRESH_TOKEN".to_string(),
            scope: "TEST_SCOPE".to_string(),
            updated_at: Utc.ymd(2022, 2, 7).and_hms(0, 1, 0),
        };

        let token_json_str = r#"{"access_token":"TEST_ACCESS_TOKEN","created_at":"2022-02-06T01:20:00+00:00","expires_at":"2022-03-06T01:20:00+00:00","id":"TEST_ID","patreon_id":"TEST_PATREON_ID","refresh_token":"TEST_REFRESH_TOKEN","scope":"TEST_SCOPE","updated_at":"2022-02-07T00:01:00+00:00"}"#;

        assert_eq!(serde_json::to_string(&token).unwrap(), token_json_str);
    }
}
