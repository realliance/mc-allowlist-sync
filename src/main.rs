use allowlist::Allowlist;
use rcon::Error;
use rcon_conn::establish_rcon_connection;

use crate::oidc::get_community_token;

mod allowlist;
mod oidc;
mod rcon_conn;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("{:?}", get_community_token().await);

    let address = "localhost:25575";
    let rcon = establish_rcon_connection(address, "test").await?;

    let mut allowlist = Allowlist(rcon);
    let res = allowlist.list().await?;

    println!("{:?}", res);

    Ok(())
}
