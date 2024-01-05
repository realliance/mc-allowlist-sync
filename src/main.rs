use crate::{config::Configuration, group::get_group_members, oidc::get_community_token};
use allowlist::Allowlist;
use anyhow::Result;
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
    let members = get_group_members(&token).await?;
    let member_len = members.len();
    println!(
        "Found {} member{} in the group to sync",
        member_len,
        if member_len == 1 { "" } else { "s" }
    );

    println!("Attempting to connect to {}", config.rcon_addr);

    let rcon = establish_rcon_connection(config.rcon_addr, &config.rcon_pass).await?;

    println!("Connected!");

    let mut allowlist = Allowlist(rcon);
    let res = allowlist.list().await?;

    println!("Current Allowlist: {:?}", res);

    Ok(())
}
