use crate::app::feature::prefers::PrefersHeaders;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Main(cx: Scope) -> impl IntoView {
    let prefers_headers_action = create_server_action::<PrefersHeaders>(cx);

    let (hue, set_hue) = create_signal(cx, 100);

    let hue_change = move |ev| {
        set_hue(event_target_value(&ev).parse::<i32>().unwrap());
    };

    let style = move || format!("--hue: {};", hue());

    view! {
        cx,

        <Html attributes=AdditionalAttributes::from(vec![ ("style", style) ]) />

        <header>
            <h3>"Settings"</h3>

            // <ActionForm action=prefers_headers_action>
            //     <select name="value">
            //         <option value="forced-colors">"Forced colors"</option>
            //         <option value="prefers-color-scheme">"Color scheme"</option>
            //         <option value="prefers-contrast">"Contrast"</option>
            //         <option value="prefers-reduced-motion">"Reduced motion"</option>
            //     </select>

            //     <button type="submit">"Go!"</button>
            // </ActionForm>

            <form id="theme-switcher">
                <b>"Color scheme"</b>
                <div></div>

                <b>"Contrast"</b>
                <div></div>

                <b>"Hue angle"</b>
                <input id="hue" name="hue" type="range" min="0" max="360" step="1" value=hue on:input=hue_change />
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
