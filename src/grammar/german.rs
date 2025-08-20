use super::{smart_options, GramClass, Word};

pub fn correct(word: &Word, answer: &String, gram_class: &GramClass) -> f32 {
    if answer.is_empty() {
        return 0.;
    }
    word.base
        .iter()
        .map(|i| match gram_class {
            _ if i.eq_ignore_ascii_case(answer) => 1.,
            GramClass::Noun => smart_options(i, answer, ["der ", "die ", "das "].into()),
            GramClass::Verb => smart_options(i, answer, ["jdn ", "jdm "].into()),
            _ => 0.0,
        })
        .fold(0., |max, val| if val > max { val } else { max })
}

#[cfg(test)]
mod test {
    use crate::grammar::*;

    #[test]
    fn german_noun() {
        let noun = Entry("die Kraft".into(), "la force".into(), GramClass::Noun);
        assert_eq!(noun.correct(&"die Kraft".into(), 0, &Lang::German), 1.);
        assert_eq!(noun.correct(&"das Kraft".into(), 0, &Lang::German), 0.5);
        assert_eq!(noun.correct(&"Kraft".into(), 0, &Lang::German), 0.5);
    }
}
