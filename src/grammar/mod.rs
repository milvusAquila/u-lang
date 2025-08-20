use json::JsonValue;

pub mod word;
pub use word::*;
pub mod english;
pub mod french;
pub mod german;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Entry(pub Word, pub Word, pub GramClass);

impl Entry {
    pub fn get(&self, element: usize) -> String {
        let word = match element {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Unavailable index"),
        };
        if word.desc.is_empty() {
            format!("{}", word)
        } else {
            format!("{} [{}]", word, &word.desc)
        }
    }
    pub fn correct(&self, answer: &String, element: usize, lang: &Lang) -> f32 {
        let word = match element {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Unavailable index"),
        };
        match *lang {
            Lang::Other if word.base.contains(answer) => 1.,
            Lang::English => english::correct(word, answer, &self.2),
            Lang::French => french::correct(word, answer, &self.2),
            Lang::German => german::correct(word, answer, &self.2),
            _ => 0.,
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", &self.0, &self.1)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
            "English" | "english" | "en" | "en_US" | "en_GB" => Self::English,
            "German" | "Deutsch" | "german" | "deutsch" | "de" | "de_DE" => Self::German,
            "French" | "Français" | "french" | "français" | "Francais" | "francais" | "fr"
            | "fr_FR" => Self::French,
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

pub fn parse(raw: &String) -> Result<([Lang; 2], Vec<Entry>), GramErr> {
    match json::parse(raw.as_str()) {
        Ok(data) if data["lang"].len() == 2 && data["list"].is_array() => {
            let lang1: Lang = data["lang"][0].as_str().unwrap_or("").into();
            let lang2: Lang = data["lang"][1].as_str().unwrap_or("").into();

            let mut list = Vec::new();
            if let JsonValue::Array(unparsed_list) = &data["list"] {
                for unparsed_entry in unparsed_list {
                    match parse_entry(unparsed_entry) {
                        Ok(entry) => list.push(entry),
                        Err(_) => return Err(GramErr::LangErr),
                    }
                }
            }
            Ok(([lang1, lang2], list))
        }
        Err(_) => Err(GramErr::Unknown),
        _ => Err(GramErr::LangErr),
    }
}

fn parse_entry(raw: &JsonValue) -> Result<Entry, GramErr> {
    let mut entry = Entry::default();
    match parse_word(&raw[0]) {
        Ok(word) => entry.0 = word,
        Err(_) => return Err(GramErr::JsonErr),
    }
    match parse_word(&raw[1]) {
        Ok(word) => entry.1 = word,
        Err(_) => return Err(GramErr::JsonErr),
    }
    match &raw[2] {
        JsonValue::Null => entry.2 = GramClass::default(),
        JsonValue::String(gram_class) => entry.2 = gram_class.into(),
        JsonValue::Short(gram_class) => entry.2 = gram_class.as_str().into(),
        _ => return Err(GramErr::JsonErr),
    }
    Ok(entry)
}

fn parse_word(raw: &JsonValue) -> Result<Word, GramErr> {
    match &raw {
        JsonValue::String(word) => Ok(word.into()),
        JsonValue::Short(word) => Ok(word.into()),
        JsonValue::Array(unparsed_words) => {
            let mut words = Vec::new();
            for unparsed_word in unparsed_words {
                match unparsed_word {
                    JsonValue::String(word) => words.push(word.as_str()),
                    JsonValue::Short(word) => words.push(word.as_str()),
                    _ => return Err(GramErr::JsonErr),
                    // _ => words.push(""),
                }
            }
            let words: Vec<String> = words.iter().map(|word| (*word).to_string()).collect();
            Ok(Word::new_list(words))
        }
        _ => return Err(GramErr::JsonErr),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GramErr {
    JsonErr,
    LangErr,
    Unknown,
}

pub fn smart_options(right: &String, answer: &str, options: Vec<&str>) -> f32 {
    let size = options[0].len();
    if options.contains(&&right[..size]) {
        if (&right[size..]).eq_ignore_ascii_case(answer) {
            0.5
        } else if answer.len() >= size + 1 {
            if (&right[size..]).eq_ignore_ascii_case(&answer[size..]) {
                0.5
            } else {
                0.0
            }
        } else {
            0.0
        }
    } else {
        0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn entry_test() {
        let entry = Entry(
            Word::new("the solution"),
            Word::new("la solution"),
            GramClass::Noun,
        );
        assert_eq!(
            entry.correct(&String::from("the solution"), 0, &Lang::English),
            1.
        );
    }
    #[test]
    fn parse_test() {
        let raw = String::from(
            r#"{
    "lang": ["english", "french"],
    "list": [
            ["yes", "oui", "adv"],
            ["no", "non", "adverb"],
            ["the work", "le travail", "noun"],
            ["the rust", "la rouille", "noun"],
            ["the solution", "la solution", "noun"],
            ["to rise", ["s'élever", "monter"], "verb"]
    ]
} "#,
        );
        println!("{}", raw);
        let parsed = parse(&raw).unwrap();
        let truth = (
            [Lang::English, Lang::French],
            vec![
                Entry("yes".into(), "oui".into(), GramClass::Adverb),
                Entry("no".into(), "non".into(), GramClass::Adverb),
                Entry("the work".into(), "le travail".into(), GramClass::Noun),
                Entry("the rust".into(), "la rouille".into(), GramClass::Noun),
                Entry("the solution".into(), "la solution".into(), GramClass::Noun),
                Entry(
                    "to rise".into(),
                    Word::new_list(vec!["s'élever".into(), "monter".into()]),
                    GramClass::Verb,
                ),
            ],
        );
        assert_eq!(parsed, truth);
    }
    #[test]
    fn read_file_test() {
        for i in
            fs::read_dir("assets").expect("Failed to open assets files (should be in /assets/*)")
        {
            let contents = fs::read_to_string(i.unwrap().path()).unwrap();
            let (langs, database) = parse(&contents).unwrap();
            for i in database {
                println!("{:?}", i);
                for j in &i.0.base {
                    assert_eq!(i.correct(&j, 0, &langs[1]), 1.0);
                }
                for j in &i.1.base {
                    assert_eq!(i.correct(&j, 1, &langs[0]), 1.0);
                }
            }
        }
    }
}
