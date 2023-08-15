use leptos::*;
use leptos_router::*;
use leptos_use::storage::use_storage;
use crate::app::feature::auth::{ AuthState, exchange };

#[derive(Params, PartialEq)]
struct RedirectParams {
    state: String,
    code: String,
    scope: String,
    prompt: String,
}

#[component]
pub fn Redirect() -> impl IntoView {
    let query = use_query::<RedirectParams>();

    let state = move || {
        query.with(|query| {
            query
                .as_ref()
                .map(|query| query.state.to_owned())
                .unwrap()
        })
    };

    let code = move || {
        query.with(|query| {
            query
                .as_ref()
                .map(|query| query.code.to_owned())
                .unwrap()
        })
    };

    let scope = move || {
        query.with(|query| {
            query
                .as_ref()
                .map(|query| query.scope.to_owned())
                .unwrap()
        })
    };

    let prompt = move || {
        query.with(|query| {
            query
                .as_ref()
                .map(|query| query.prompt.to_owned())
                .unwrap()
        })
    };

    
    let (s, _, _) = use_storage("auth-state", AuthState::default());
    let auth_state = s();

    log!("{}", auth_state.nonce);

    
    spawn_local(async move {
        let _ = exchange(code(), auth_state.nonce, auth_state.pkce_verifier).await;
    });

    // cfg_if::cfg_if! {
    //     if #[cfg(not(feature="ssr"))] {
    //         let (s, _, _) = use_storage("auth-state", AuthState::default());
    //         let auth_state = s();
        
    //         log!("{}", auth_state.nonce);

            
    //         spawn_local(async move {
    //             let _ = exchange(code(), auth_state.nonce, auth_state.pkce_verifier).await;
    //         });
    //     }
    // }

    view! {
        <table>
            <tbody>
                <tr>
                    <td><strong>state</strong></td>
                    <td>{state}</td>
                </tr>

                <tr>
                    <td><strong>code</strong></td>
                    <td>{code}</td>
                </tr>
                
                <tr>
                    <td><strong>scope</strong></td>
                    <td>{scope}</td>
                </tr>
                
                <tr>
                    <td><strong>prompt</strong></td>
                    <td>{prompt}</td>
                </tr>
            </tbody>
        </table>
    }
}