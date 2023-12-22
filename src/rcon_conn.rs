use rcon::{Connection, Error};
use tokio::net::{TcpStream, ToSocketAddrs};

pub async fn establish_rcon_connection<A: ToSocketAddrs>(
    addr: A,
    password: &str,
) -> Result<Connection<TcpStream>, Error> {
    <Connection<TcpStream>>::builder()
        .enable_minecraft_quirks(true)
        .connect(addr, password)
        .await
}
