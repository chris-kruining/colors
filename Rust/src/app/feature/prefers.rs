use core::fmt;
use leptos::*;
use std::default;
use std::str::FromStr;
use strum::EnumString;

#[derive(Default, Debug, Clone)]
pub enum ReducedMotion {
    #[default]
    NoPreference,
    Reduce,
}

impl fmt::Display for ReducedMotion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReducedMotion::NoPreference => write!(f, "no-preference"),
            ReducedMotion::Reduce => write!(f, "reduce"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum Contrast {
    #[default]
    NoPreference,
    More,
    Less,
    Custom,
}

impl fmt::Display for Contrast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Contrast::NoPreference => write!(f, "no-preference"),
            Contrast::More => write!(f, "more"),
            Contrast::Less => write!(f, "less"),
            Contrast::Custom => write!(f, "custom"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum ColorScheme {
    #[default]
    NoPreference,
    Dark,
    Light,
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorScheme::NoPreference => write!(f, "no-preference"),
            ColorScheme::Dark => write!(f, "dark"),
            ColorScheme::Light => write!(f, "light"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum ForcedColors {
    #[default]
    NoPreference,
    Active,
}

impl fmt::Display for ForcedColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForcedColors::NoPreference => write!(f, "no-preference"),
            ForcedColors::Active => write!(f, "active"),
        }
    }
}

#[derive(Debug, Clone, EnumString)]
pub enum Prefers {
    #[strum(serialize = "prefers-contrast")]
    Contrast(Contrast),
    #[strum(serialize = "prefers-color-scheme")]
    ColorScheme(ColorScheme),
    #[strum(serialize = "prefers-reduced-motion")]
    ReducedMotion(ReducedMotion),
    #[strum(serialize = "forced-colors")]
    ForcedColors(ForcedColors),
}

// impl fmt::Display for Prefers {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Prefers::Contrast(contrast) => write!(f, "prefers-contrast: {contrast}"),
//             Prefers::ColorScheme(color_scheme) => write!(f, "prefers-color-scheme: {color_scheme}"),
//             Prefers::ForcedColors(forced_colors) => write!(f, "forced-colors: {forced_colors}"),
//         }
//     }
// }

#[server(PrefersHeaders, "/api")]
pub async fn prefers_headers(value: String) -> Result<bool, ServerFnError> {
    use actix_web::HttpRequest;

    let value = Prefers::from_str(&value);

    if let Some(request) = use_context::<HttpRequest>() {
        println!("WOOT");

        let _ = dbg!(request);
        let _ = dbg!(value);
    }

    Ok(false)
}
