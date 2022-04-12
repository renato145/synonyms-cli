use crate::request_handling::{get_web_synonyms, ResponseItem};
use itertools::Itertools;
use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    Cell, Row, Table,
};
use spinners::{Spinner, Spinners};

#[derive(Debug)]
pub struct Synonyms {
    pub category: String,
    pub words: Vec<Word>,
}

impl From<ResponseItem> for Synonyms {
    fn from(item: ResponseItem) -> Self {
        let category = item.list.category.replace('(', "").replace(')', "");
        let words = get_words(item.list.synonyms);
        Self { category, words }
    }
}

pub async fn get_synonyms(
    word: &str,
    language: &str,
    api_key: &str,
) -> anyhow::Result<Vec<Synonyms>> {
    let spinner = Spinner::new(Spinners::CircleHalves, "Getting synonyms...".to_string());
    let res = get_web_synonyms(word, language, api_key).await;
    spinner.stop_with_message(termion::clear::CurrentLine.to_string());
    Ok(res?.response.into_iter().map(Synonyms::from).collect())
}

const MAX_COLS: usize = 12;
const WORD_SIZE: usize = 20;

pub fn show_synonyms(synonyms: Vec<Synonyms>) {
    let (terminal_cols, _) = termion::terminal_size().unwrap();
    let cols = (terminal_cols as usize / WORD_SIZE).min(MAX_COLS);
    for (category, synonyms) in synonyms
        .into_iter()
        .group_by(|s| s.category.clone())
        .into_iter()
    {
        let mut table = Table::new();
        table.add_row(Row::new(vec![Cell::new(&format!(
            "Category: {}",
            category
        ))
        .with_hspan(cols)
        .style_spec("b")]));
        synonyms
            .into_iter()
            .flat_map(|s| s.words)
            .chunks(cols)
            .into_iter()
            .map(|chunk| Row::new(chunk.map(|w| Cell::new(&w.get_printable())).collect()))
            .for_each(|row| {
                table.add_row(row);
            });

        let format = FormatBuilder::new()
            .padding(1, 1)
            .separator(LinePosition::Top, LineSeparator::new('─', '┬', '┌', '┐'))
            .separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '└', '┘'))
            .separator(LinePosition::Intern, LineSeparator::new(' ', ' ', ' ', ' '))
            .build();
        table.set_format(format);
        table.printstd();
        println!();
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

    pub fn get_printable(&self) -> String {
        format!(
            "{}\n{}",
            self.content,
            ansi_term::Style::new()
                .dimmed()
                .paint(format!("({})", self.description))
        )
    }
}

pub fn get_words(s: String) -> Vec<Word> {
    let mut words = Vec::new();
    let mut word_buffer = Vec::new();
    for part in s.split('|') {
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
        let input = serde_json::from_value::<ResponseItem>(serde_json::json!({
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
        let words = Synonyms::from(input);
        assert_eq!(&words.category, "adj");
        assert_eq!(words.words, expected);
    }
}
