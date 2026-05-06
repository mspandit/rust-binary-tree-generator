use std::fmt::{ Debug, Display };
use std::hash::Hash;
use crate::grammar::Binary;
use crate::{state::State, grammar::Grammar};
mod context;
mod state;
mod grammar;
trait Token: Clone + Display + Default + Eq + Hash {}

impl Token for char {}
impl Token for &str {}
impl Token for String {}

trait Tokenizeable<T: Token> {
    fn tokenize(self) -> impl Iterator<Item = T>;
}

impl Tokenizeable<char> for &str {
    fn tokenize(self) -> impl Iterator<Item = char> {
        self.chars()
    }
}

impl<'a> Tokenizeable<&'a str> for Vec<&'a str> {
    fn tokenize(self) -> impl Iterator<Item = &'a str> {
        self.into_iter()
    }
}

impl Tokenizeable<String> for Vec<String> {
    fn tokenize(self) -> impl Iterator<Item = String> {
        self.into_iter()
    }
}

fn generate_contexts<T: Token, S: Clone + Eq + Hash + 'static + Debug>(input_sequence: impl Iterator<Item = T>, grammar: &dyn Grammar<T, S>)
-> State<S> {
    input_sequence.fold(
        State::default(),
        |gen_state, input| {
            gen_state.process(input, grammar)
        }
    )
}

fn generate<T: Token + Debug + 'static, S: Clone + Eq + Hash + 'static + Debug>(input_sequence: impl Tokenizeable<T>, grammar: &dyn Grammar<T, S>)
-> Vec<S> {
    generate_contexts(input_sequence.tokenize(), grammar)
        .filter_contexts()
        .tops::<T>()
}

fn main() {
    let x = generate("abcdef", &Binary::default());
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    let _word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    // let x = generate(word_sequence, &Grammar::sentence());
    // println!("{} trees", x.len());
    // for t in x {
    //     println!("{:?}", t);
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary() {
        let x = generate("a", &Binary::default());
        assert_eq!(1, x.len(), "{x:?}");
        let x = generate("ab", &Binary::default());
        assert_eq!(1, x.len(), "{x:?}");
        let x = generate("abc", &Binary::default());
        assert_eq!(2, x.len(), "{x:?}");
        let x = generate("abcd", &Binary::default());
        assert_eq!(5, x.len(), "{x:?}");
        let x = generate("abcde", &Binary::default());
        assert_eq!(14, x.len(), "{x:?}");
        let x = generate("abcdef", &Binary::default());
        assert_eq!(42, x.len(), "{x:?}");
    }

    // #[test]
    // fn test_zero_characters() {
    //     let x = generate_contexts("".chars(), &Grammar::expression());
    //     assert_eq!(1, x.len());
    //     let x = generate("", &Grammar::expression());
    //     assert_eq!(0, x.len());
    // }

    // #[test]
    // fn test_zero_words() {
    //     let x = generate_contexts(vec![].into_iter(), &Grammar::sentence());
    //     assert_eq!(1, x.len());
    //     let x = generate(vec![], &Grammar::sentence());
    //     assert_eq!(0, x.len());
    // }

    // #[test]
    // fn test_one_character() {
    //     let x = generate_contexts("1".chars(), &Grammar::expression());
    //     assert_eq!(1, x.len());
    //     let x = generate("1", &Grammar::expression());
    //     assert_eq!(1, x.len());
    // }

    // #[test]
    // fn test_one_word() {
    //     let x = generate_contexts(vec!["the".to_string()].into_iter(), &Grammar::sentence());
    //     assert_eq!(1, x.len());
    //     let x = generate(vec!["the".to_string()], &Grammar::sentence());
    //     assert_eq!(1, x.len());
    // }

    // #[test]
    // fn test_two_characters() {
    //     let x = generate_contexts("-1".chars(), &Grammar::expression());
    //     assert_eq!(2, x.len(), "{x:?}");
    //     let x = generate("-1", &Grammar::expression());
    //     assert_eq!(1, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_two_words() {
    //     let x = generate_contexts(vec!["the".to_string(), "cat".to_string()].into_iter(), &Grammar::sentence());
    //     assert_eq!(2, x.len(), "{x:?}");
    //     let x = generate(vec!["the".to_string(), "cat".to_string()], &Grammar::sentence());
    //     assert_eq!(1, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_three_characters() {
    //     let x = generate("1+3", &Grammar::expression());
    //     assert_eq!(1, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_four_characters() {
    //     let x = generate("-1+2*4", &Grammar::expression());
    //     assert_eq!(5, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_six_words() {
    //     let x = generate(vec!["the".to_string(), "cat".to_string(), "sat".to_string(), "on".to_string(), "the".to_string(), "mat".to_string()], &Grammar::sentence());
    //     assert_eq!(1, x.len(), "{x:?}");
    // }
}