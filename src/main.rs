use crate::grammar::{Gram, noun, noun_phrase, recursive, sentence};
mod grammar;

fn main() {
    let factorial = recursive(&|factorial: &Gram<i32, ()>, x| {
        if *x <= 0 {
            println!("Blastoff!");
        } else {
            println!("Counting down: {}", *x);
            factorial.apply(&(x - 1));
        }
        vec![]
    });
    factorial.apply(&3);

    let x = noun().parse(& vec!["cat".to_string()]);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    println!("---------------------");
    let x = noun().parse(& vec!["mat".to_string()]);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    println!("---------------------");
    let word_sequence = vec!["the", "cat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let x = noun_phrase().parse(&word_sequence);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    println!("---------------------");
    let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let x = sentence().parse(&word_sequence);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
}

#[cfg(test)]
mod test {

use super::*;

    // #[test]
    // fn test_binary1() {
    //     let x = binary_string().parse(&"a".chars().collect::<Vec<char>>());
    //     assert_eq!(1, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary2() {
    //     let x = generate("ab", binary_string());
    //     assert_eq!(1, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary3() {
    //     let x = generate("abc", binary_string());
    //     assert_eq!(2, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary4() {
    //     let x = generate("abcd", binary_string());
    //     assert_eq!(5, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary5() {
    //     let x = generate("abcde", binary_string());
    //     assert_eq!(14, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary6() {
    //     let x = generate("abcdef", binary_string());
    //     assert_eq!(42, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_zero_characters() {
    //     let input = vec![];
    //     let x = expression()
    //     .parse(& input);
    //     assert_eq!("[Cont(Grammar::Shift)]", format!("{:?}", x));
    // }

    // #[test]
    // fn test_one_character() {
    //     let input = vec!['1'];
    //     let x = expression().parse(& input);
    //     assert_eq!("[Term(1), Cont(Grammar::Shift)]", format!("{:?}", x));
    // }

    // #[test]
    // fn test_one_word() {
    //     let x = generate_contexts(vec!["the".to_string()].into_iter(), sentence());
    //     assert_eq!(2, x.len());
    //     let x = generate(vec!["the".to_string()], sentence());
    //     assert_eq!(0, x.len());
    // }

    // #[test]
    // fn test_two_characters() {
    //     let x = generate_contexts("-1".chars(), expression());
    //     assert_eq!(3, x.len(), "{x:?}");
    //     let x = generate_contexts("1+".chars(), expression());
    //     assert_eq!(3, x.len(), "{x:?}");
    //     let x = generate("-1", expression());
    //     assert_eq!(1, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_two_words() {
    //     let x = generate_contexts(vec!["the".to_string(), "cat".to_string()].into_iter(), sentence());
    //     assert_eq!(3, x.len(), "{x:?}");
    //     let x = generate(vec!["the".to_string(), "cat".to_string()], sentence());
    //     assert_eq!(0, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_three_characters() {
    //     let x = generate("1+3", expression());
    //     assert_eq!(1, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_four_characters() {
    //     let x = generate("-1+2*4", expression());
    //     assert_eq!(5, x.len(), "{x:?}");
    // }

    #[test]
    fn test_six_words() {
        let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let x = sentence().parse(&word_sequence);
        assert_eq!(1, x.len(), "{x:?}");
    }
}