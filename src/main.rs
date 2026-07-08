use crate::grammar::{Grammar, noun, noun_phrase, recursive, sentence};
mod grammar;

fn main() {
    let countdown = recursive(&|factorial: &Grammar<i32, ()>, x: &i32| {
        if *x <= 0 {
            println!("Blastoff!");
        } else {
            println!("Counting down: {}", *x);
            factorial.apply(&(x - 1));
        }
        vec![]
    });
    countdown.apply(&3);

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

use crate::grammar::{binary_string, expression};

use super::*;

    #[test]
    fn test_binary1() {
        let x = binary_string().parse(&"a".chars().collect::<Vec<char>>());
        assert_eq!(2, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary2() {
        let input = "ab".chars().collect();
        let x = binary_string().parse(&input);
        assert_eq!(2, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary3() {
        let input = "abc".chars().collect();
        let x = binary_string().parse(&input);
        assert_eq!(2, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary4() {
        let input = "abcd".chars().collect();
        let x = binary_string().parse(&input);
        assert_eq!(5, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary5() {
        let input = "abcde".chars().collect();
        let x = binary_string().parse(&input);
        assert_eq!(14, x.len(), "{x:?}");
    }
    #[test]
    fn test_binary6() {
        let input = "abcdef".chars().collect();
        let x = binary_string().parse(&input);
        assert_eq!(42, x.len(), "{x:?}");
    }

    #[test]
    fn test_zero_characters() {
        let input = vec![];
        let x = expression()
        .parse(& input);
        assert_eq!("[Cont(Grammar::Shift)]", format!("{:?}", x));
    }

    // #[test]
    // fn test_one_character() {
    //     let input = vec!['1'];
    //     let x = expression().parse(& input);
    //     assert_eq!("[Nonterminal(1), Cont(Grammar::Shift)]", format!("{:?}", x));
    // }

    #[test]
    fn test_one_word() {
        let input = vec!["the".to_string()];
        let x = sentence().parse(& input);
        assert_eq!("[Nonterminal(the), Cont(Grammar::Shift)]", format!("{:?}", x));
    }

    // #[test]
    // fn test_two_characters() {
    //     let input = "-1".chars().collect();
    //     let x = expression().parse(& input);
    //     assert_eq!("[Nonterminal(-1), Cont(Grammar::Shift)]", format!("{:?}", x));
    //     let input = "1+".chars().collect();
    //     let x = expression().parse(& input);
    //     assert_eq!("[Nonterminal(1), Cont(Grammar::Shift)]", format!("{:?}", x));
    // }

    #[test]
    fn test_two_words() {
        let input = vec!["the".to_string(), "cat".to_string()];
        let x = sentence().parse(&input);
        assert_eq!("[Nonterminal(the), Nonterminal(cat), Cont(Grammar::Shift)]", format!("{:?}", x));
    }

    // #[test]
    // fn test_three_characters() {
    //     let input = "1+3".chars().collect();
    //     let x = expression().parse(& input);
    //     assert_eq!(1, x.len(), "{x:?}");
    // }

    // #[test]
    // fn test_four_characters() {
    //     let input = "-1+2*4".chars().collect();
    //     let x = expression().parse(& input);
    //     assert_eq!(5, x.len(), "{x:?}");
    // }

    #[test]
    fn test_six_words() {
        let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let x = sentence().parse(&word_sequence);
        assert_eq!(1, x.len(), "{x:?}");
    }
}