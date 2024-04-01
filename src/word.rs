pub struct Entry(pub Word, pub Word, pub GramClass);

pub enum Word {
    One(String),
    List(Vec<String>),
}

pub enum GramClass {
    Noun,
    Verb,
}

impl Entry {
    pub fn get(&self, lang: usize) -> String {
        let word = &self.0;
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
}

impl Into<Word> for &str {
    fn into(self) -> Word {
        Word::One(String::from(self))
    }
}
