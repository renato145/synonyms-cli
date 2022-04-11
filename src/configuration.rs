use anyhow::{anyhow, Context, Result};
use dialoguer::{Confirm, Input};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub api_key: String,
}

pub fn get_settings() -> Result<Settings> {
    let config_file = ProjectDirs::from("", "", "synonyms")
        .ok_or(anyhow!("Couldn't retrieve configuration path"))?
        .config_dir()
        .to_path_buf()
        .join("config.yaml");

    if !config_file.exists()
        && Confirm::new()
            .show_default(true)
            .with_prompt(format!(
                "Configuration file ({:?}) does not exists. Do you want to create one?",
                config_file
            ))
            .report(true)
            .interact()?
    {
        let parent = config_file.parent().unwrap();
        if !parent.exists() {
            create_dir_all(parent)?;
        }
        let api_key = Input::new()
            .with_prompt(
                "Enter the api key for Thesaurus \
                 get one at https://thesaurus.altervista.org/mykey)",
            )
            .report(true)
            .interact_text()?;
        let settings = Settings { api_key };
        let yaml_file = File::create(config_file.clone())?;
        serde_yaml::to_writer(yaml_file, &settings)?;
        println!("Saved settings to {:?}\n{:?}", config_file, settings);
    }
    let settings = config::Config::builder()
        .add_source(config::File::from(config_file).required(true))
        .build()?;
    settings
        .try_deserialize()
        .context("Failed to deserialize settings.")
}
