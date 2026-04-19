use std::fmt::Display;
use std::hash::Hash;
use crate::{binary_tree::BinaryTree, generator_state::GeneratorState, grammar::Grammar};
mod stack;
mod binary_tree;
mod generator_state;
mod grammar;
mod baker;
mod bayesian_pcfg_induction;
mod berwick_pilato;
trait Token: Clone + Display + Default + Eq + Hash {}

impl Token for char {}
impl Token for &str {}

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

fn generate_stacks<T: Token>(input_sequence: impl Iterator<Item = T>, grammar: &Grammar<T>)
-> GeneratorState<T> {
    input_sequence.fold(
        GeneratorState::default(),
        |gen_state, input| {
            gen_state.process(input, grammar)
        }
    )
}

fn generate<T: Token>(input_sequence: impl Tokenizeable<T>, grammar: &Grammar<T>)
-> Vec<BinaryTree<T>> {
    generate_stacks(input_sequence.tokenize(), grammar)
        .filter_stacks()
        .tops()
}

fn main() {
    let x = generate("-1+2*4", &Grammar::expression());
    println!("{} trees", x.len());
    for t in x {
        println!("{}", t);
    }
    let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"];
    let x = generate(word_sequence, &Grammar::sentence());
    println!("{} trees", x.len());
    for t in x {
        println!("{}", t);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zero_characters() {
        let x = generate_stacks("".chars(), &Grammar::expression());
        assert_eq!(1, x.len());
        let x = generate("", &Grammar::expression());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_zero_words() {
        let x = generate_stacks(vec![].into_iter(), &Grammar::sentence());
        assert_eq!(1, x.len());
        let x = generate(vec![], &Grammar::sentence());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_one_character() {
        let x = generate_stacks("1".chars(), &Grammar::expression());
        assert_eq!(1, x.len());
        let x = generate("1", &Grammar::expression());
        assert_eq!(1, x.len());
    }

    #[test]
    fn test_one_word() {
        let x = generate_stacks(vec!["the"].into_iter(), &Grammar::sentence());
        assert_eq!(1, x.len());
        let x = generate(vec!["the"], &Grammar::sentence());
        assert_eq!(1, x.len());
    }

    #[test]
    fn test_two_characters() {
        let x = generate_stacks("-1".chars(), &Grammar::expression());
        assert_eq!(3, x.len(), "{x:?}");
        let x = generate("-1", &Grammar::expression());
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_two_words() {
        let x = generate_stacks(vec!["the", "cat"].into_iter(), &Grammar::sentence());
        assert_eq!(2, x.len(), "{x:?}");
        let x = generate(vec!["the", "cat"], &Grammar::sentence());
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_three_characters() {
        let x = generate("1+3", &Grammar::expression());
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_four_characters() {
        let x = generate("-1+2*4", &Grammar::expression());
        assert_eq!(5, x.len(), "{x:?}");
    }

    #[test]
    fn test_six_words() {
        let x = generate(vec!["the", "cat", "sat", "on", "the", "mat"], &Grammar::sentence());
        assert_eq!(1, x.len(), "{x:?}");
    }
}