pub mod feature;
pub mod route;

use crate::app::feature::{
    dark_mode_toggle::DarkModeToggle,
    auth::Auth,
};
use crate::app::route::{
    main::Main,
    auth::redirect::Redirect
};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    cfg_if! {
        if #[cfg(feature="ssr")] {
            use leptos_actix::*;
            use http::{HeaderName, HeaderValue};

            if let Some(res_options) = use_context::<ResponseOptions>() {
                res_options.append_header(
                    HeaderName::from_lowercase(b"accept-ch").unwrap(),
                    HeaderValue::from_str("Sec-CH-Prefers-Color-Scheme, Sec-CH-Prefers-Contrast").unwrap()
                );

            }
        }
    }

    view! {
        <Stylesheet id="leptos" href="/pkg/colors.css" />
        <Title text="Welcome to Leptos"/>

        <Router>
            <Routes>
                <Route path="" view=Layout>
                    <Route path="" view=Main />
                    <Route path="auth/redirect" view=Redirect />
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
fn Layout() -> impl IntoView {
    view! {
        <nav>
            <Auth />
            <DarkModeToggle />
        </nav>

        <Outlet />
    }
}