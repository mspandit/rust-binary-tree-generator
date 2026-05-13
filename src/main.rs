use std::fmt::{ Debug, Display };
use std::hash::Hash;
use crate::state::State;
use crate::grammar::{binary_string, expression, Grammar, sentence, Start};
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

fn generate_contexts<T, S>(input_sequence: impl Iterator<Item = T>, grammar: Grammar<T, S>)
-> State<T, S>
where T: Token + Debug + 'static, S: Start<S> + Clone + Debug + 'static {
    input_sequence.fold(
        State::new(grammar.clone()),
        |gen_state, input| {
            gen_state.apply(& input)
        }
    )
}

fn generate<T, S>(input_sequence: impl Tokenizeable<T>, grammar: Grammar<T, S>)
-> Vec<S>
where T: Token + Debug + 'static, S: Start<S> + Clone + Debug + 'static {
    generate_contexts(input_sequence.tokenize(), grammar)
        .single_contexts()
        .tops()
}

fn main() {
    let x = generate("abcdef", binary_string());
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let x = generate(word_sequence, sentence());
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    let x = generate("-1+2*4", expression());
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
}

#[cfg(test)]
mod test {

use super::*;

    #[test]
    fn test_binary1() {
        let x = generate("a", binary_string());
        assert_eq!(1, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary2() {
        let x = generate("ab", binary_string());
        assert_eq!(1, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary3() {
        let x = generate("abc", binary_string());
        assert_eq!(2, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary4() {
        let x = generate("abcd", binary_string());
        assert_eq!(5, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary5() {
        let x = generate("abcde", binary_string());
        assert_eq!(14, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary6() {
        let x = generate("abcdef", binary_string());
        assert_eq!(42, x.len(), "{x:?}");
    }

    #[test]
    fn test_zero_characters() {
        let x = generate_contexts("".chars(), expression());
        assert_eq!(1, x.len());
        let x = generate("", expression());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_zero_words() {
        let x = generate_contexts(vec![].into_iter(), sentence());
        assert_eq!(1, x.len());
        let x = generate(vec![], sentence());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_one_character() {
        let x = generate_contexts("1".chars(), expression());
        assert_eq!(2, x.len());
        let x = generate("1", expression());
        assert_eq!(1, x.len());
    }

    #[test]
    fn test_one_word() {
        let x = generate_contexts(vec!["the".to_string()].into_iter(), sentence());
        assert_eq!(2, x.len());
        let x = generate(vec!["the".to_string()], sentence());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_two_characters() {
        let x = generate_contexts("-1".chars(), expression());
        assert_eq!(3, x.len(), "{x:?}");
        let x = generate_contexts("1+".chars(), expression());
        assert_eq!(3, x.len(), "{x:?}");
        let x = generate("-1", expression());
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_two_words() {
        let x = generate_contexts(vec!["the".to_string(), "cat".to_string()].into_iter(), sentence());
        assert_eq!(3, x.len(), "{x:?}");
        let x = generate(vec!["the".to_string(), "cat".to_string()], sentence());
        assert_eq!(0, x.len(), "{x:?}");
    }

    #[test]
    fn test_three_characters() {
        let x = generate("1+3", expression());
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_four_characters() {
        let x = generate("-1+2*4", expression());
        assert_eq!(5, x.len(), "{x:?}");
    }

    #[test]
    fn test_six_words() {
        let x = generate(vec!["the".to_string(), "cat".to_string(), "sat".to_string(), "on".to_string(), "the".to_string(), "mat".to_string()], sentence());
        assert_eq!(1, x.len(), "{x:?}");
    }
}