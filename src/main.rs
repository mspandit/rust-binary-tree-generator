use std::fmt::Display;

use crate::{binary_tree::BinaryTree, generator_state::GeneratorState};
mod stack;
mod binary_tree;
mod generator_state;
trait Token: Clone + Display + Default{}

impl Token for char {}
impl Token for &str {}

fn generate_stacks<T: Token>(input_sequence: impl Iterator<Item = T>)
-> GeneratorState<T> {
    input_sequence.fold(
        GeneratorState::default(),
        |gen_state, input| {
            gen_state.process(input)
        }
    )
}

fn generate<T: Token>(input_sequence: impl Iterator<Item = T>)
-> Vec<BinaryTree<T>> {
    generate_stacks(input_sequence)
        .filter_stacks()
        .tops()
}

fn main() {
    let x = generate("abcde".chars());
    println!("{} trees", x.len());
    for t in x {
        println!("{}", t);
    }
    let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"];
    let x = generate(word_sequence.into_iter());
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
        let x = generate_stacks("".chars());
        assert_eq!(1, x.len());
        let x = generate("".chars());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_one_character() {
        let x = generate_stacks("a".chars());
        assert_eq!(1, x.len());
        let x = generate("a".chars());
        assert_eq!(1, x.len());
    }

    #[test]
    fn test_two_characters() {
        let x = generate_stacks("ab".chars());
        println!("{x:?}");
        assert_eq!(2, x.len());
        let x = generate("ab".chars());
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_three_characters() {
        let x = generate("abc".chars());
        assert_eq!(2, x.len(), "{x:?}");
    }

    #[test]
    fn test_four_characters() {
        let x = generate("abcd".chars());
        assert_eq!(5, x.len());
    }
}