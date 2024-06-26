use rcon::{Connection, Error};
use tokio::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
}

pub struct Allowlist(pub Connection<TcpStream>);

impl Allowlist {
    pub async fn list(&mut self) -> Result<Vec<Player>, Error> {
        let response = self.0.cmd("whitelist list").await?;

        println!("{}", response);

        if &response == "There are no whitelisted players" {
            Ok(Vec::new())
        } else {
            let player_list = response.split(": ").last().unwrap();
            let players = player_list
                .split(", ")
                .map(|x| Player {
                    name: x
                        .strip_suffix("\r\n")
                        .or(x.strip_suffix("\n"))
                        .unwrap_or(x)
                        .to_string(),
                })
                .collect();
            Ok(players)
        }
    }

    pub async fn add(&mut self, name: String) -> Result<(), Error> {
        let res = self.0.cmd(&format!("whitelist add {name}")).await?;

        println!("{res}");
        Ok(())
    }

    pub async fn remove(&mut self, name: String) -> Result<(), Error> {
        let res = self.0.cmd(&format!("whitelist remove {name}")).await?;

        println!("{res}");
        Ok(())
    }
}
