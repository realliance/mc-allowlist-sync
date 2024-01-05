use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "camelCase")]
pub struct GroupUser {
    pub id: String,
    pub username: String,
}

pub async fn get_group_members(token: &str) -> Vec<GroupUser> {
    let client = Client::new();
    serde_json::from_str(&client
        .get("https://community.realliance.net/api/groups/8080b382-8900-4152-90e9-0aef930c5ef3/members")
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()).unwrap()
}
