use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::app::feature::prefers::{PrefersHeaders};

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    let prefers_headers_action = create_server_action::<PrefersHeaders>(cx);
    
    view! {
        cx,
        
        <div>
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
        </div>
    }
}