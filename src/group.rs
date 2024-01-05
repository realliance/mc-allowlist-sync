use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "camelCase")]
pub struct GroupUser {
    pub id: String,
    pub username: String,
}

pub async fn get_group_members(token: &str) -> Result<Vec<GroupUser>> {
    let client = Client::new();
    let members_req = client
    .get("https://community.realliance.net/api/groups/8080b382-8900-4152-90e9-0aef930c5ef3/members")
    .bearer_auth(token)
    .send()
    .await?
    .text()
    .await?;

    let members = serde_json::from_str(&members_req)?;

    Ok(members)
}
