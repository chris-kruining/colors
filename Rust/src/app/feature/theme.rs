use cfg_if::cfg_if;
use leptos::*;
use wasm_cookies::*;

cfg_if! {
    if #[cfg(feature="ssr")] {
        pub fn get_hue() -> i32 {
            use_context::<actix_web::HttpRequest>()
                .and_then(|req| {
                    req.cookie("hue")?
                        .value()
                        .parse::<i32>()
                        .ok()
                })
                .unwrap_or(0)
        }
    }
    else {
        pub fn get_hue() -> i32 {
            use wasm_bindgen::JsCast;

            let doc = document().dyn_into::<web_sys::HtmlDocument>().unwrap();

            cookies::get(&doc.cookie().unwrap_or_default(), "hue")
                .and_then(|value| value.ok()?.parse::<i32>().ok())
                .unwrap_or(100)
        }
    }
}

#[server(ThemeSetHue, "/api")]
pub async fn set_hue(hue: i32) -> Result<i32, ServerFnError> {
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_actix::{ResponseOptions, ResponseParts};

    let response = use_context::<ResponseOptions>().expect("to have leptos_actix::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("hue={hue}; Path=/")).expect("to create header value"),
    );
    response_parts.headers = headers;

    std::thread::sleep(std::time::Duration::from_millis(250));

    response.overwrite(response_parts);
    Ok(hue)
}
