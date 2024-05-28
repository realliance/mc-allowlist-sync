use anyhow::{anyhow, Result};
use reqwest::{header::USER_AGENT, Client};
use serde::Deserialize;

use crate::allowlist::Player;

#[derive(Deserialize, Debug)]
#[serde(rename = "camelCase")]
pub struct UserConnections {
    pub user_id: Option<String>,
    #[serde(rename = "minecraft_uuid")]
    pub minecraft_uuid: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "camelCase")]
pub struct GroupUser {
    pub id: String,
    pub username: String,
    pub connections: Option<UserConnections>,
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

#[derive(Debug, Deserialize)]
pub struct PlayerData {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct DbData {
    pub player: PlayerData,
}

#[derive(Debug, Deserialize)]
pub struct PlayerDb {
    pub data: DbData,
}

impl GroupUser {
    pub async fn try_into_player(self) -> Result<Player> {
        let minecraft_uuid = self
            .connections
            .ok_or(anyhow!("Connections missing"))?
            .minecraft_uuid
            .ok_or(anyhow!("Minecraft UUID Missing"))?;

        let client = Client::new();
        let player_db_res = client
            .get(format!(
                "https://playerdb.co/api/player/minecraft/{minecraft_uuid}"
            ))
            .header(USER_AGENT, "github.com/realliance/mc-modpack")
            .send()
            .await?
            .json::<PlayerDb>()
            .await?;

        Ok(Player {
            name: player_db_res.data.player.username,
        })
    }
}
