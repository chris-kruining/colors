pub mod feature;

use crate::app::feature::{
    dark_mode_toggle::{DarkModeToggle, DarkModeToggleProps},
    prefers::PrefersHeaders,
};
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

    let prefers_headers_action = create_server_action::<PrefersHeaders>(cx);

    view! {
        cx,

        <Stylesheet id="leptos" href="/pkg/colors.css" />
        <Title text="Welcome to Leptos"/>

        <Router>
            <nav>
                <DarkModeToggle />
            </nav>

            <main>
                <fieldset title="Get the prefered:">
                    <ActionForm action=prefers_headers_action>
                        <select name="value">
                            <option value="color-scheme">"Color scheme"</option>
                            <option value="contrast">"Contrast"</option>
                            <option value="reduced-motion">"Color scheme"</option>
                        </select>
                        <button type="submit">"Go!"</button>
                    </ActionForm>
                </fieldset>
            </main>
        </Router>
    }
}
