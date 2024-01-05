use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, IssuerUrl, ResourceOwnerPassword, ResourceOwnerUsername,
};

use openidconnect::OAuth2TokenResponse;

use anyhow::Result;

pub async fn get_community_token(
    client_id: String,
    username: String,
    password: String,
) -> Result<String> {
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://id.realliance.net/application/o/community/".to_string())?,
        async_http_client,
    )
    .await?;

    let client =
        CoreClient::from_provider_metadata(provider_metadata, ClientId::new(client_id), None);

    Ok(client
        .exchange_password(
            &ResourceOwnerUsername::new(username),
            &ResourceOwnerPassword::new(password),
        )
        .request_async(async_http_client)
        .await?
        .access_token()
        .secret()
        .clone())
}
