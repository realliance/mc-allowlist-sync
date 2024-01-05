use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub rcon_addr: String,
    pub rcon_pass: String,
    pub oidc_client_id: String,
    pub oidc_user: String,
    pub oidc_pass: String,
}
