use anyhow::{Context, Result};
use serde::Deserialize;

const BASE_URL: &str = "https://thesaurus.altervista.org/thesaurus/v1";

#[derive(Debug, Deserialize)]
pub struct WebResponse {
    pub response: Vec<ResponseItem>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseItem {
    pub list: ItemList,
}

#[derive(Debug, Deserialize)]
pub struct ItemList {
    pub category: String,
    pub synonyms: String,
}

pub async fn get_web_synonyms(word: &str, language: &str, api_key: &str) -> Result<WebResponse> {
    let response = reqwest::get(format!(
        "{}?word={}&language={}&key={}&output=json",
        BASE_URL, word, language, api_key
    ))
    .await
    .with_context(|| format!("Failed to do request to: {BASE_URL}"))?
    .json::<serde_json::Value>()
    .await
    .context("Failed to deserialize response")?;
    let synonyms = {
        if let Some(err) = response.get("error") {
            anyhow::bail!("{}", err)
        }
        serde_json::from_value::<WebResponse>(response).context("Invalid response")?
    };
    Ok(synonyms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::get_settings;
    use claim::assert_err;

    fn get_key() -> String {
        get_settings(false).unwrap().api_key
    }

    #[tokio::test]
    async fn correctly_report_errors() {
        let res = get_web_synonyms("asadasdasdasd", "en_US", &get_key()).await;
        println!("{:?}", res);
        assert_err!(res);
    }

    #[tokio::test]
    async fn correctly_get_results() {
        let res = get_web_synonyms("good", "en_US", &get_key()).await.unwrap();
        println!("{:?}", res);
    }
}
