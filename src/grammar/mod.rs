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

#[derive(Debug)]
pub enum Lang {
    English,
    French,
    German,
    Other,
}
