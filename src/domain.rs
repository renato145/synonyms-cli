use crate::request_handling::{ResponseItem, WebResponse};

#[derive(Debug)]
pub struct Synonyms {
    pub words: Vec<Word>,
}

impl From<WebResponse> for Synonyms {
    fn from(web_response: WebResponse) -> Self {
        let words = web_response
            .response
            .into_iter()
            .flat_map(get_words)
            .collect::<Vec<_>>();
        Self { words }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Word {
    pub content: String,
    pub description: String,
}

impl Word {
    pub fn new(content: String, description: String) -> Self {
        Self {
            content,
            description,
        }
    }
}

pub fn get_words(item: ResponseItem) -> Vec<Word> {
    let mut words = Vec::new();
    let mut word_buffer = Vec::new();
    for part in item.list.synonyms.split('|') {
        let part_splits = part.split('(').collect::<Vec<_>>();
        word_buffer.push(part_splits[0].trim().to_string());
        if let Some(&desc) = part_splits.get(1) {
            for content in word_buffer.drain(..) {
                words.push(Word::new(content, desc.replace(')', "").to_string()))
            }
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_parse_response_item() {
        let input = serde_json::from_value(serde_json::json!({
            "list": {
                "category": "(adj)",
                "synonyms": "potential|latent (similar term)|actual (antonym)"
            }
        }))
        .unwrap();
        let expected = vec![
            Word::new("potential".to_string(), "similar term".to_string()),
            Word::new("latent".to_string(), "similar term".to_string()),
            Word::new("actual".to_string(), "antonym".to_string()),
        ];
        assert_eq!(get_words(input), expected);
    }
}
