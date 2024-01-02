use std::env;

use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, ClientSecret, IssuerUrl, RedirectUrl, ResourceOwnerPassword, ResourceOwnerUsername,
};

use openidconnect::OAuth2TokenResponse;

pub async fn get_community_token() -> String {
    let client_id = env::var("CLIENT_ID").unwrap();
    let username = env::var("USER_NAME").unwrap();
    let password = env::var("PASSWORD").unwrap();

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://id.realliance.net/application/o/community-api/".to_string())
            .unwrap(),
        async_http_client,
    )
    .await
    .unwrap();

    // Create an OpenID Connect client by specifying the client ID, client secret, authorization URL
    // and token URL.
    let client =
        CoreClient::from_provider_metadata(provider_metadata, ClientId::new(client_id), None)
            // Set the URL the user will be redirected to after the authorization process.
            .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string()).unwrap());

    client
        .exchange_password(
            &ResourceOwnerUsername::new(username),
            &ResourceOwnerPassword::new(password),
        )
        .request_async(async_http_client)
        .await
        .unwrap()
        .access_token()
        .secret()
        .clone()
}
