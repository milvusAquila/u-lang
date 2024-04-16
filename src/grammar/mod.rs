use core::panic;

pub mod word;
pub use word::*;

#[derive(Debug)]
pub struct Entry(pub Word, pub Word, pub GramClass);

impl Entry {
    pub fn get(&self, lang: usize) -> String {
        let word = match lang {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Unavailable index"),
        };
        match word {
            Word::One(content) => content.to_string(),
            Word::List(content) => {
                let mut formatted = String::new();
                for i in content {
                    formatted += format!("{} / ", i).as_str();
                }
                formatted
            }
        }
    }
    pub fn correct(&self, answer: &String) -> f32 {
        // TODO: add some grammar tolerences (`to` or not before verb)
        match &self.0 {
            Word::One(word) => {
                println!("{}={}", &word, &answer);
                if word == answer {
                    1.
                } else {
                    0.
                }
            }
            Word::List(words) => {
                if words.contains(answer) {
                    1.
                } else {
                    0.
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Lang {
    English,
    French,
    German,
    Other,
}

impl<'a> Into<&'a str> for Lang {
    fn into(self) -> &'a str {
        match self {
            Self::English => "English",
            Self::German => "Deutsch",
            Self::French => "Français",
            Self::Other => "Other",
        }
    }
}

impl From<&str> for Lang {
    fn from(value: &str) -> Self {
        match value {
            "English" | "english" => Self::English,
            "German" | "Deutsch" | "german" | "deutsch" => Self::German,
            "French" | "Français" | "french" | "français" | "Francais" | "francais" => {
                Self::French
            }
            _ => Self::Other,
        }
    }
}
impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Lang::English => "English",
            Lang::German => "Deutsch",
            Lang::French => "Français",
            Lang::Other => "Other",
        };
        write!(f, "{}", string)
    }
}
/* impl From<Lang> for String {
    fn from(value: Lang) -> Self {
        value.into()
    }
} */
