use super::{smart_options, GramClass, Word};

pub fn correct(word: &Word, answer: &String, gram_class: &GramClass) -> f32 {
    if answer.is_empty() {
        return 0.;
    }
    word.base
        .iter()
        .map(|i| match gram_class {
            _ if i.eq_ignore_ascii_case(answer) => 1.0,
            GramClass::Noun => smart_options(i, answer, ["le ", "la "].into()),
            _ => 0.,
        })
        .fold(0., |max, val| if val > max { val } else { max })
}
