use super::{GramClass, Word};

pub fn correct(word: &Word, answer: &String, gram_class: &GramClass) -> f32 {
    word.base
        .iter()
        .map(|i| match gram_class {
            _ if i.eq_ignore_ascii_case(answer) => 1.,
            GramClass::Verb
                if (&i.chars().collect::<Vec<char>>()[..3]
                    == "to ".chars().collect::<Vec<char>>()
                    && answer == &i[3..]) =>
            {
                1.
            }
            GramClass::Noun if (&i[..4] == "the " && answer == &i[4..]) => 1.,
            GramClass::Noun if (&i[..2] == "a " && answer == &i[2..]) => 1.,
            _ => 0.,
        })
        .fold(0., |max, val| if val > max { val } else { max })
}

#[cfg(test)]
mod test {
    use crate::grammar::*;

    #[test]
    fn english_verb() {
        let verb = Entry("to rise".into(), "s'élever".into(), GramClass::Verb);
        assert_eq!(verb.correct(&"to rise".into(), 0, &Lang::English), 1.);
        assert_eq!(verb.correct(&"rise".into(), 0, &Lang::English), 1.);
        assert_eq!(verb.correct(&"rse".into(), 0, &Lang::English), 0.);
    }
    #[test]
    fn english_noun() {
        let noun = Entry("the solution".into(), "la solution".into(), GramClass::Noun);
        assert_eq!(noun.correct(&"the solution".into(), 0, &Lang::English), 1.);
        assert_eq!(noun.correct(&"solution".into(), 0, &Lang::English), 1.);
        assert_eq!(noun.correct(&"solutio".into(), 0, &Lang::English), 0.);
    }
}
