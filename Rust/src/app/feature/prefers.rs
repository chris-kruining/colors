use core::fmt;
use leptos::*;
use leptos_meta::{Meta, MetaProps};
use leptos_router::{ActionForm, ActionFormProps};

pub enum ReducedMotion {
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

pub enum Contrast {
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

pub enum ColorScheme {
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

pub enum ForcedColors {
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

pub enum Prefers {
    Contrast(Contrast),
    ColorScheme(ColorScheme),
    ForcedColors(ForcedColors),
}

impl fmt::Display for Prefers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefers::Contrast(contrast) => write!(f, "prefers-contrast: {contrast}"),
            Prefers::ColorScheme(color_scheme) => write!(f, "prefers-color-scheme: {color_scheme}"),
            Prefers::ForcedColors(forced_colors) => write!(f, "forced-colors: {forced_colors}"),
        }
    }
}

#[server(PrefersHeaders, "/api")]
pub async fn prefers_headers(cx: Scope) -> Result<bool, ServerFnError> {
    use actix_web::HttpRequest;

    if let Some(request) = use_context::<HttpRequest>(cx) {
        println!("WOOT");
    }

    Ok(false)
}
