use cfg_if::cfg_if;

use anyhow::anyhow;
use leptos::*;
use leptos_use::storage::use_storage;
use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreProviderMetadata
};
use openidconnect::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    IssuerUrl, Nonce, PkceCodeChallenge, RedirectUrl, Scope
};

use openidconnect::reqwest::async_http_client;
use serde::{Serialize, Deserialize};

type MyClient = openidconnect::Client<
    openidconnect::EmptyAdditionalClaims, 
    openidconnect::core::CoreAuthDisplay, 
    openidconnect::core::CoreGenderClaim, 
    openidconnect::core::CoreJweContentEncryptionAlgorithm, 
    openidconnect::core::CoreJwsSigningAlgorithm, 
    openidconnect::core::CoreJsonWebKeyType, 
    openidconnect::core::CoreJsonWebKeyUse, 
    openidconnect::core::CoreJsonWebKey, 
    openidconnect::core::CoreAuthPrompt, 
    openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>, 
    openidconnect::StandardTokenResponse<
        openidconnect::IdTokenFields<
            openidconnect::EmptyAdditionalClaims, 
            openidconnect::EmptyExtraTokenFields, 
            openidconnect::core::CoreGenderClaim, 
            openidconnect::core::CoreJweContentEncryptionAlgorithm, 
            openidconnect::core::CoreJwsSigningAlgorithm, 
            openidconnect::core::CoreJsonWebKeyType
        >, 
        openidconnect::core::CoreTokenType
    >, 
    openidconnect::core::CoreTokenType, 
    openidconnect::StandardTokenIntrospectionResponse<
        openidconnect::EmptyExtraTokenFields, 
        openidconnect::core::CoreTokenType
    >, 
    openidconnect::core::CoreRevocableToken, 
    openidconnect::StandardErrorResponse<openidconnect::RevocationErrorResponseType>
>;

async fn get_client() -> Result<MyClient, anyhow::Error> {
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(
            "https://accounts.google.com".to_string(),
        )?,
        async_http_client,
    ).await?;

    Ok(
        CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(
                "339822783127-422ci0ntiq2u49d80vu95ackrmetvtqi.apps.googleusercontent.com".to_string(),
            ),
            Some(ClientSecret::new(
                "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".to_string(),
            )),
        )
        .set_redirect_uri(RedirectUrl::new("http://localhost:3000/auth/redirect".to_string())?)
    )
}

pub async fn start() -> Result<(url::Url, openidconnect::Nonce, openidconnect::PkceCodeVerifier), anyhow::Error> {
    let client: MyClient = get_client().await?;

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // Set the desired scopes.
        .add_scope(Scope::new("email".to_string()))
        // .add_scope(Scope::new("read".to_string()))
        // .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    Ok((auth_url, nonce, pkce_verifier))
}

pub async fn exchange(code: String, _nonce: String, pkce_verifier: String) -> Result<(), anyhow::Error> {
    use openidconnect::TokenResponse;

    let pkce_verifier = openidconnect::PkceCodeVerifier::;

    let client = get_client().await?;

    let token_response = client
        .exchange_code(AuthorizationCode::new(code))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;

    let _id_token = token_response.id_token().ok_or_else(|| anyhow!("Server did not return an ID token"))?;

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AuthState {
    pub nonce: String,
    pub pkce_verifier: String,
}


cfg_if! {
    if #[cfg(feature="ssr")] {
        async fn test() -> Result<(), anyhow::Error> {
            Ok(())
        }
    } else {
        async fn test() -> Result<(), anyhow::Error> {

            let (auth_url, nonce, pkce_verifier) = start().await?;

            let (state, set_state, _) = use_storage("auth-state", AuthState::default());

            set_state(AuthState { nonce: nonce.secret().to_string(), pkce_verifier: pkce_verifier.secret().to_string() });

            log!("Browse to: {}", auth_url);

            Ok(())

            // let claims = id_token.claims(&client.id_token_verifier(), &nonce)?;
        
            // // Verify the access token hash to ensure that the access token hasn't been substituted for
            // // another user's.
            // if let Some(expected_access_token_hash) = claims.access_token_hash() {
            //     let actual_access_token_hash = AccessTokenHash::from_token(token_response.access_token(), &id_token.signing_alg()?)?;
                
            //     if actual_access_token_hash != *expected_access_token_hash {
            //         return Err(anyhow!("Invalid access token"));
            //     }
            // }
        
            // // The authenticated user's identity is now available. See the IdTokenClaims struct for a
            // // complete listing of the available claims.
            // log!(
            //     "User {} with e-mail address {} has authenticated successfully",
            //     claims.subject().as_str(),
            //     claims
            //         .email()
            //         .map(|email| email.as_str())
            //         .unwrap_or("<not provided>"),
            // );
        
            // // If available, we can use the UserInfo endpoint to request additional information.
        
            // // The user_info request uses the AccessToken returned in the token response. To parse custom
            // // claims, use UserInfoClaims directly (with the desired type parameters) rather than using the
            // // CoreUserInfoClaims type alias.
            // let userinfo: CoreUserInfoClaims = client
            //     .user_info(token_response.access_token().to_owned(), None)
            //     .map_err(|err| anyhow!("No user info endpoint: {:?}", err))?
            //     .request_async(async_http_client).await
            //     .map_err(|err| anyhow!("Failed requesting user info: {:?}", err))?;
        
            // Ok(userinfo)
        }
    }
}

#[server(Auth, "/api")]
pub async fn auth() -> Result<String, ServerFnError> {
    match test().await {
        Err(msg) => Ok(msg.to_string()),
        Ok(_) => Ok("".to_string())
    }
}

#[component]
pub fn Auth() -> impl IntoView {
    let on_click = move |_| {
        spawn_local(async move {
            let _ = auth().await;
            let _ = test().await;
        });

    };

    view! {
        <button on:click=on_click>Log in</button>
    }
}
