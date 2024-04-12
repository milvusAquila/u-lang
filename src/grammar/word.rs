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

impl Into<Word> for &str {
    fn into(self) -> Word {
        Word::One(String::from(self))
    }
}
impl Into<Word> for String {
    fn into(self) -> Word {
        Word::One(self)
    }
}
impl Into<Word> for Vec<&str> {
    fn into(self) -> Word {
        Word::List(self.iter().map(|word| String::from(*word)).collect())
    }
}
impl Into<Word> for Vec<String> {
    fn into(self) -> Word {
        Word::List(self)
    }
}
