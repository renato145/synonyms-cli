mod configuration;
mod domain;
mod request_handling;

use crate::{configuration::get_settings, domain::Synonyms, request_handling::get_web_synonyms};
use anyhow::Result;
use clap::Parser;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Opts {
    word: String,
    #[clap(short, long, default_value_t=String::from("en"))]
    language: String,
}

const LANGUAGES: &str = "en, fr, cs, es, el, da, de, hu, it, no, pl, pt, ro, ru, sk";
const LANGUAGES_: &str ="en_US, fr_FR, cs_CZ, es_ES, el_GR, da_DK, de_DE, hu_HU, it_IT, no_NO, pl_PL, pt_PT, ro_RO, ru_RU, sk_SK";
static LANGUAGE_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (a, b) in LANGUAGES.split(',').zip(LANGUAGES_.split(',')) {
        m.insert(a.to_string(), b.to_string());
    }
    m
});

#[tokio::main]
async fn main() -> Result<()> {
    let Opts { word, language } = Opts::parse();
    let language = LANGUAGE_MAP
        .get(&language)
        .unwrap_or_else(|| panic!("Invalid language, select one of: {}", LANGUAGES));
    let settings = get_settings(true)?;

    let response = get_web_synonyms(&word, language, &settings.api_key).await?;
    let synonyms = Synonyms::from(response);

    println!("{:#?}", synonyms);
    Ok(())
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Opts::command().debug_assert()
}
