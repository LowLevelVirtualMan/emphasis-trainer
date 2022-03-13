mod parse;

use std::fmt::Display;

use rand::Rng;

use crate::util;

use self::parse::ParseError;

/// Struct that manages whole logic of trainer.
pub struct Model {
    cur: usize,
    words: Vec<Word>,
}

impl Model {
    /// Create new model.
    pub fn new() -> Result<Self, Vec<ParseError>> {
        let data = include_str!("./data.txt");
        let words = parse::parse(data)?;
        Ok(Model { cur: 0, words })
    }

    /// Get new word.
    pub fn next(&self) -> Word {
        let mut rng = rand::thread_rng();
        let i: usize = rng.gen_range(0..self.words.len());
        self.words[i].clone()
    }

    /// Get words with the same group and rule.
    pub fn seealso(&self, word: &Word) -> Vec<Word> {
        let group = match word.group {
            Some(group) => group,
            None => return Vec::new(),
        };
        self.words.iter()
            .filter(|w| w.group.map(|g| g == group).unwrap_or(false))
            .map(|w| w.clone())
            .filter(|w| w != word)
            .collect()
    }

    /// Get word with the same group but opposite rule.
    pub fn opposite(&self, word: &Word) -> Vec<Word> {
        let group = match word.group {
            Some(group) => group,
            None => return Vec::new(),
        };
        self.words.iter()
            .filter(|w| w.group.map(|g| g.0 == !group.0 && g.1 == group.1).unwrap_or(false))
            .map(|w| w.clone())
            .filter(|w| w != word)
            .collect()
    }
}

/// Correct way to set emphasis at `word`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    /// Word in lowercase.
    pub word: String,
    /// Detail that defines correct emphasis.
    pub detail: Option<String>,
    /// Position of correct emphasis.
    pub emphasis: usize,
    /// Words with same seealso value are shown after failure.
    pub group: Option<(bool, u64)>,
    /// Explanation with presented tag shown after failute.
    pub explanation: Option<String>,
}

impl Word {
    pub fn new(word: &str, emphasis: usize) -> Self {
        Word {
            word: word.to_lowercase(),
            detail: None,
            emphasis,
            group: None,
            explanation: None,
        }
    }
    
    pub fn with_detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.trim().to_string());
        self
    }
    
    pub fn with_group(mut self, group: &str, inverted: bool) -> Self {
        self.group = Some((inverted, fxhash::hash64(&group.to_lowercase())));
        self
    }

    pub fn with_explanation(mut self, explanation: impl ToString) -> Self {
        self.explanation = Some(explanation.to_string());
        self
    }

    pub fn variants(&self) -> Vec<Variant> {
        util::get_vowel_positions(&self.word)
            .into_iter()
            .map(|emphasis| Variant {
                emphasis,
                word: self.word.clone(),
                detail: self.detail.clone(),
            })
            .collect()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = util::uppercase_letter(&self.word, self.emphasis);
        if let Some(detail) = &self.detail {
            write!(f, "{} {}", word, detail)
        } else {
            write!(f, "{}", word)
        }
    }
}

/// Variant is possibly incorrect way of setting emphasis at word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub emphasis: usize,
    pub word: String,
    pub detail: Option<String>,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = self.word.replace('ё', "е");
        let word = util::uppercase_letter(&word, self.emphasis);
        if let Some(detail) = &self.detail {
            write!(f, "{} {}", word, detail)
        } else {
            write!(f, "{}", word)
        }
    }
}