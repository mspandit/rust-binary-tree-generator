use std::fmt::{ Debug, Display };
use std::hash::Hash;
use crate::grammar::{binary_string, expression, sentence};
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
    let x= binary_string().parse(&"abcdef".into());
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    let x = sentence().parse(&vec!["the", "cat", "sat", "on", "the", "mat"].into());
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    let x = expression().parse(&"-1+2*4".into());
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
        let x = binary_string().parse(&"a".into());
        assert_eq!(1, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary2() {
        let x = binary_string().parse(&"ab".into());
        assert_eq!(2, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary3() {
        let x = binary_string().parse(&"abc".into());
        assert_eq!(1, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary4() {
        let x = binary_string().parse(&"abcd".into());
        assert_eq!(5, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary5() {
        let x = binary_string().parse(&"abcde".into());
        assert_eq!(14, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary6() {
        let x = binary_string().parse(&"abcdef".into());
        assert_eq!(42, x.len(), "{x:?}");
    }

    #[test]
    fn test_zero_characters() {
        let x = binary_string().parse(&"".into());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_zero_words() {
        let x = sentence().parse(&vec![].into());
        assert_eq!(0, x.len());
    }


    #[test]
    fn test_one_word() {
        let x = sentence().parse(&vec!["the"].into());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_two_characters() {
        let x = expression().parse(&"-1".into());
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_two_words() {
        let x = sentence().parse(&vec!["cat"].into());
        assert_eq!(0, x.len(), "{x:?}");
    }

    #[test]
    fn test_three_characters() {
        let x = expression().parse(&"1+3".into());
        assert_eq!(1, x.len(), "{x:?}");
    }

    #[test]
    fn test_four_characters() {
        let x = expression().parse(&"-1+2*4".into());
        assert_eq!(5, x.len(), "{x:?}");
    }

    #[test]
    fn test_six_words() {
        let x = sentence().parse(&vec!["the", "cat", "sat", "on", "the", "mat"].into());
        assert_eq!(1, x.len(), "{x:?}");
    }
}