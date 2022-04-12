mod configuration;
mod domain;
mod request_handling;

use crate::configuration::get_settings;
use ansi_term::{Color, Style};
use anyhow::Result;
use clap::Parser;
use dialoguer::Input;
use domain::{get_synonyms, show_synonyms};
use once_cell::sync::Lazy;
use std::{collections::HashMap, process};

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Opts {
    word: Option<String>,
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
    let language = LANGUAGE_MAP.get(&language).unwrap_or_else(|| {
        eprintln!("Invalid language, select one of: {}", LANGUAGES);
        process::exit(1);
    });
    let settings = get_settings(true)?;

    if let Some(word) = word {
        let synonyms = get_synonyms(&word, language, &settings.api_key).await?;
        show_synonyms(synonyms);
    } else {
        start_interactive(language, &settings.api_key).await?;
    }

    Ok(())
}

async fn start_interactive(language: &str, api_key: &str) -> Result<()> {
    let prompt = format!(
        "{}",
        Style::new().bold().paint("Enter a word (use 'q' to quit)")
    );
    loop {
        let word: String = Input::new()
            .with_prompt(&prompt)
            .report(true)
            .interact_text()?;
        if word == "q" {
            break;
        }
        match get_synonyms(&word, language, api_key).await {
            Ok(synonyms) => {
                show_synonyms(synonyms);
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    Style::new().fg(Color::Red).paint(format!("Error: {}", e))
                );
            }
        }
    }
    Ok(())
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Opts::command().debug_assert()
}
