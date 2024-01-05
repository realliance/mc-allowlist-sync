use std::env;

use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, IssuerUrl, ResourceOwnerPassword, ResourceOwnerUsername,
};

use openidconnect::OAuth2TokenResponse;

pub async fn get_community_token() -> String {
    let client_id = env::var("CLIENT_ID").expect("Ensure CLIENT_ID is provided as an environment variable that defined the Realliance ID Client to use.");
    let username = env::var("USER_NAME").expect(
        "Ensure USER_NAME is provided as the username of the service account to auth against.",
    );
    let password = env::var("PASSWORD").expect(
        "Ensure PASSWORD is provided as the app token for the service account to auth against.",
    );

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://id.realliance.net/application/o/community/".to_string()).unwrap(),
        async_http_client,
    )
    .await
    .unwrap();

    let client =
        CoreClient::from_provider_metadata(provider_metadata, ClientId::new(client_id), None);

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
