use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Entry {
    TextEntry(Box<str>),
    Subsection(Section),
}

impl Entry {
    pub fn as_text(&self) -> Option<&str> {
        let Entry::TextEntry(text) = self else {
            return None;
        };
        Some(&text)
    }

    pub fn as_subsection(&self) -> Option<&Section> {
        let Entry::Subsection(section) = self else {
            return None;
        };
        Some(&section)
    }
}

pub type Section = IndexMap<Box<str>, Entry>;
