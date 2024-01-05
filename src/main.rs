use std::env;

use allowlist::Allowlist;
use rcon::Error;
use rcon_conn::establish_rcon_connection;

use crate::{group::get_group_members, oidc::get_community_token};

mod allowlist;
mod group;
mod oidc;
mod rcon_conn;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("== Realliance MC Allowlist Sync ==");
    let rcon_address = env::var("RCON_ADDR")
        .expect("Ensure RCON_ADDR is provided to define where the rcon server is located.");
    let rcon_password = env::var("RCON_PASS")
        .expect("Ensure RCON_PASS is provided to auth against the rcon server.");

    let token = get_community_token().await;
    let members = get_group_members(&token).await;
    let member_len = members.len();
    println!(
        "Found {} member{} in the group to sync",
        member_len,
        if member_len == 1 { "" } else { "s" }
    );

    println!("Attempting to connect to {}", rcon_address);

    let rcon = establish_rcon_connection(rcon_address, &rcon_password).await?;

    println!("Connected!");

    let mut allowlist = Allowlist(rcon);
    let res = allowlist.list().await?;

    println!("Current Allowlist: {:?}", res);

    Ok(())
}
