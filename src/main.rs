use std::collections::HashSet;

use crate::{config::Configuration, group::get_group_members, oidc::get_community_token};
use allowlist::Allowlist;
use anyhow::{anyhow, Result};
use futures::future::join_all;
use rcon_conn::establish_rcon_connection;

mod allowlist;
mod config;
mod group;
mod oidc;
mod rcon_conn;

#[tokio::main]
async fn main() -> Result<()> {
    println!("== Realliance MC Allowlist Sync ==\n");
    println!("Checking environment variables...");
    let config = envy::from_env::<Configuration>()?;

    let token =
        get_community_token(config.oidc_client_id, config.oidc_user, config.oidc_pass).await?;
    let members = get_group_members(&token)
        .await?
        .into_iter()
        .filter(|x| {
            x.connections
                .as_ref()
                .is_some_and(|x| x.minecraft_uuid.is_some())
        })
        .collect::<Vec<_>>();
    let member_len = members.len();
    println!(
        "Found {} member{} in the group to sync",
        member_len,
        if member_len == 1 { "" } else { "s" }
    );

    let members_as_players = members
        .into_iter()
        .map(|member| async { member.try_into_player().await });

    let members_as_players = join_all(members_as_players).await;

    let (players, failures): (Vec<_>, Vec<_>) =
        members_as_players.into_iter().partition(|x| x.is_ok());

    if let Some(first_fail) = failures.into_iter().next() {
        return Err(anyhow!(first_fail.unwrap_err()));
    }

    let expected_players = players.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();

    let expected_player_lut = expected_players
        .clone()
        .into_iter()
        .map(|x| x.name)
        .collect::<HashSet<String>>();

    println!("Attempting to connect to {}", config.rcon_addr);

    let rcon = establish_rcon_connection(config.rcon_addr, &config.rcon_pass).await?;

    println!("Connected!");

    let mut allowlist = Allowlist(rcon);
    let current_players = allowlist.list().await?;

    let current_player_lut = current_players
        .clone()
        .into_iter()
        .map(|x| x.name)
        .collect::<HashSet<String>>();

    // If the current player lust doesnt contain an expected player, means we need to add
    let additions = expected_players
        .iter()
        .filter(|player| !current_player_lut.contains(&player.name));

    // If the current players isn't in the expected players, means we need to remove
    let deletions = current_players
        .iter()
        .filter(|player| !expected_player_lut.contains(&player.name));

    for addition in additions {
        println!("Adding {}", addition.name);
        allowlist.add(addition.name.clone()).await?;
    }

    for removal in deletions {
        println!("Removing {}", removal.name);
        allowlist.remove(removal.name.clone()).await?;
    }

    println!("Done!");

    Ok(())
}
