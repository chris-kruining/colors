use leptos::ServerFnError;

use leptos::*;

#[server(AuthRedirect, "/api", "GetJson", "auth/redirect")]
pub async fn redirect() -> Result<String, ServerFnError> {
    Ok("Great success".to_string())
}