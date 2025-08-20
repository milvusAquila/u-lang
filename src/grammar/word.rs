use regex::Regex;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Word {
    pub base: Vec<String>, // many options can be separated by /
    pub desc: String,      // between [] in json, display to give context
}

impl Word {
    pub fn new(one: impl Into<String>) -> self::Word {
        Word {
            base: vec![one.into()],
            ..Default::default()
        }
    }
    pub fn new_list(list: impl Into<Vec<String>>) -> self::Word {
        Word {
            base: list.into(),
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.base.len();
        let mut string = String::new();
        if len >= 2 {
            for i in &self.base[..(len - 1)] {
                string += format!("{} / ", i).as_str();
            }
        }
        string += &self.base[&len - 1].as_str();
        write!(f, "{}", string)
    }
}

impl Into<Word> for &str {
    fn into(self) -> Word {
        Word {
            base: vec![self.into()],
            ..Default::default()
        }
    }
}

impl Into<Word> for &String {
    fn into(self) -> Word {
        let mut word = Word::default();
        let re_desc = Regex::new(r"\[.+\]").unwrap();
        // Remove description between []
        let base = if let Some(desc) = re_desc.find(&self) {
            word.desc = self[desc.range().start + 1..desc.range().end - 1]
                .trim()
                .to_string();
            self[0..(desc.range().start)].trim()
        } else {
            self
        };
        let re_split = Regex::new(r"\s/\s").unwrap();
        let base: Vec<String> = re_split.split(base).map(|i| i.to_string()).collect();
        word.base = base;
        word
    }
}

impl Into<Word> for &json::short::Short {
    fn into(self) -> Word {
        (&self.to_string()).into()
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum GramClass {
    Adjectiv,
    Adverb,
    Noun,
    Verb,
    #[default]
    Other,
}

impl Into<GramClass> for &str {
    fn into(self) -> GramClass {
        match self {
            "Adjectiv" | "adjectiv" | "Adj" | "adj" | "Adjectif" | "adjectif" => {
                GramClass::Adjectiv
            }
            "Adverb" | "adverb" | "Adv" | "adv" | "Adverbe" | "adverbe" => GramClass::Adverb,
            "Noun" | "noun" | "Nom" | "nom" | "n" => GramClass::Noun,
            "Verb" | "verb" | "Verbe" | "verbe" | "v" => GramClass::Verb,
            _ => GramClass::Other,
        }
    }
}

impl Into<GramClass> for &String {
    fn into(self) -> GramClass {
        self.as_str().into()
    }
}
