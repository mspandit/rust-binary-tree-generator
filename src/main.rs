use std::fmt::{ Debug, Display };
use std::hash::Hash;
use crate::grammar::{binary_string, expression, sentence, Stack};
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



fn main() {
    let x= binary_string().parse(&Stack("abcdef".to_string().chars().rev().collect()));
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    let word_sequence = Stack(vec!["the", "cat", "sat", "on", "the", "mat"].iter().map(|s| s.to_string()).collect::<Vec<String>>());
    let x = sentence().parse(&word_sequence);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    let x = expression().parse(& Stack("-1+2*4".to_string().chars().rev().collect()));
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
        let x = binary_string().parse(&Stack("a".to_string().chars().rev().collect()));
        assert_eq!(1, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary2() {
        let x = binary_string().parse(&Stack("ab".to_string().chars().rev().collect()));
        assert_eq!(1, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary3() {
        let x = binary_string().parse(&Stack("abc".to_string().chars().rev().collect()));
        assert_eq!(2, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary4() {
        let x = binary_string().parse(&Stack("abcd".to_string().chars().rev().collect()));
        assert_eq!(5, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary5() {
        let x = binary_string().parse(&Stack("abcde".to_string().chars().rev().collect()));
        assert_eq!(14, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary6() {
        let x = binary_string().parse(&Stack("abcdef".to_string().chars().rev().collect()));
        assert_eq!(42, x.len(), "{x:?}");
    }

    #[test]
    fn test_zero_characters() {
        let x = binary_string().parse(&Stack("".to_string().chars().rev().collect()));
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_zero_words() {
        let x = sentence().parse(&Stack(vec![]));
        assert_eq!(0, x.len());
    }


    #[test]
    fn test_one_word() {
        let x = sentence().parse(&Stack(vec!["the".to_string()]));
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_two_characters() {
        let x = expression().parse(&Stack("-1".to_string().chars().rev().collect()));
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_two_words() {
        let x = sentence().parse(&Stack(vec!["cat".to_string(), "the".to_string()]));
        assert_eq!(0, x.len(), "{x:?}");
    }

    #[test]
    fn test_three_characters() {
        let x = expression().parse(&Stack("1+3".to_string().chars().rev().collect()));
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_four_characters() {
        let x = expression().parse(&Stack("-1+2*4".to_string().chars().rev().collect()));
        assert_eq!(5, x.len(), "{x:?}");
    }

    #[test]
    fn test_six_words() {
        let x = sentence().parse(&Stack(vec!["the", "cat", "sat", "on", "the", "mat"].into_iter().map(str::to_string).collect()));
        assert_eq!(1, x.len(), "{x:?}");
    }
}