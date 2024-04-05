use core::panic;

#[derive(Debug)]
pub struct Entry(pub Word, pub Word, pub GramClass);

#[derive(Debug)]
pub enum Word {
    One(String),
    List(Vec<String>),
}

#[derive(Debug)]
pub enum GramClass {
    Adverb,
    Noun,
    Verb,
}

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
        match &self.0 {
            Word::One(word) => if word == answer { 1. } else { 0. },
            Word::List(words) => if words.contains(answer) { 1. } else { 0. },
        }
    }
}

impl Into<Word> for &str {
    fn into(self) -> Word {
        Word::One(String::from(self))
    }
}
