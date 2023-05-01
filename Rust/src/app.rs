pub mod feature;
pub mod route;

use crate::app::feature::{
    dark_mode_toggle::{DarkModeToggle, DarkModeToggleProps},
    prefers::PrefersHeaders,
};
use crate::app::route::main::{Main, MainProps};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    cfg_if! {
        if #[cfg(feature="ssr")] {
            use leptos_actix::*;
            use http::{HeaderName, HeaderValue};

            if let Some(res_options) = use_context::<ResponseOptions>(cx) {
                res_options.append_header(
                    HeaderName::from_lowercase(b"accept-ch").unwrap(),
                    HeaderValue::from_str("Sec-CH-Prefers-Color-Scheme, Sec-CH-Prefers-Contrast").unwrap()
                );

            }
        }
    }

    view! {
        cx,

        <Stylesheet id="leptos" href="/pkg/colors.css" />
        <Title text="Welcome to Leptos"/>

        <Router>
            <nav>
                <DarkModeToggle />
            </nav>

            <Routes>
                <Route path="" view=move |cx| view! { cx, <Main/> } />
            </Routes>
        </Router>
    }
}
