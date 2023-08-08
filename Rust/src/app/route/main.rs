use crate::app::feature::prefers::PrefersHeaders;
use crate::app::feature::theme::{get_hue as initial_hue, set_hue as set_hue_action};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Main() -> impl IntoView {

    // let editor_content = create_rw_signal(cx, String::new());
    
    // let classes = PapelitoClasses {
    //     actionbar: "rte-actionbar".to_string(),
    //     button: "rte-button".to_string(),
    //     content: "rte-content".to_string(),
    //     selected: "rte-button-selected".to_string(),
    //     editor: "rte-editor".to_string(),
    // };
    
    // //  Use the ActionsBuilder struct to build the action bar (it is a optional parameter)
    // //  let actions = ActionsBuilder::new().with_bold().with_heading1().build();
    // let actions = ActionsBuilder::new().with_default_actions().build();









    let prefers_headers_action = create_server_action::<PrefersHeaders>();
    // let set_hue_action = create_server_action::<ThemeSetHue>();

    let (hue, set_hue) = create_signal(initial_hue());

    let hue_change = move |ev| {
        set_hue(event_target_value(&ev).parse::<i32>().unwrap());
    };

    let hue_changed = move |ev| {
        spawn_local(async move {
            let _ = set_hue_action(event_target_value(&ev).parse::<i32>().unwrap()).await;
        });
    };

    let style = move || format!("--hue: {};", hue());

    view! {
        <Html attributes=AdditionalAttributes::from(vec![ ("style", style) ]) />

        // <Papelito actions=actions content_signal=editor_content classes=classes key="my_unique_key".to_string()/>

        <header>
            <h3>"Settings"</h3>

            <ActionForm action=prefers_headers_action>
                <select name="value">
                    <option value="forced-colors">"Forced colors"</option>
                    <option value="prefers-color-scheme">"Color scheme"</option>
                    <option value="prefers-contrast">"Contrast"</option>
                    <option value="prefers-reduced-motion">"Reduced motion"</option>
                </select>

                <button type="submit">"Go!"</button>
            </ActionForm>

            <form id="theme-switcher">
                <b>"Color scheme"</b>
                <div></div>

                <b>"Contrast"</b>
                <div></div>

                <b>"Hue angle"</b>
                <input id="hue" name="hue" type="range" min="0" max="360" step="1" value=hue on:input=hue_change on:pointerup=hue_changed />
            </form>
        </header>

        <main>
            <section>
                <div class="surface-samples">
                    <div class="surface1 rad-shadow">1</div>
                    <div class="surface2 rad-shadow">2</div>
                    <div class="surface3 rad-shadow">3</div>
                    <div class="surface4 rad-shadow">4</div>
                </div>
            </section>

            <section>
                <div class="text-samples">
                    <h1 class="text1">
                        <span class="swatch brand rad-shadow"></span>
                        "Brand"
                    </h1>
                    <h1 class="text1">
                        <span class="swatch primary rad-shadow" title="foreground"></span>
                        <span class="swatch primary rad-shadow" title="background"></span>
                        "Primary accent"
                    </h1>
                    <h1 class="text1">
                        <span class="swatch secondary rad-shadow" title="foreground"></span>
                        <span class="swatch secondary rad-shadow" title="background"></span>
                        "Secondary accent"
                    </h1>
                    <h1 class="text1">
                        <span class="swatch text1 rad-shadow"></span>
                        "Text Color 1"
                    </h1>
                    <h1 class="text2">
                        <span class="swatch text2 rad-shadow"></span>
                        "Text Color 2"
                    </h1>
                    <br />
                    <p class="text1">"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."</p>
                    <p class="text2">"Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."</p>
                </div>
            </section>

            <section class="status-samples">
                <h1>"Status colors"</h1>

                <h2 class="text1">
                    <span class="block info rad-shadow"></span>
                    "Info"
                </h2>
                <h2 class="text1">
                    <span class="block success rad-shadow"></span>
                    "Success"
                </h2>
                <h2 class="text1">
                    <span class="block warning rad-shadow"></span>
                    "Warning"
                </h2>
                <h2 class="text1">
                    <span class="block failure rad-shadow"></span>
                    "Failure"
                </h2>
            </section>
        </main>
    }
}
