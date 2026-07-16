use std::rc::Rc;

use crate::{expression::item, grammar::{Grammar}};

// This overflows the stack during grammar definition
pub fn infinite_reduce1() -> Grammar<char, String> {
    Grammar::Reduce(vec![infinite_reduce1()])
}
// This overflows the stack during grammar application
pub fn infinite_reduce2() -> Grammar<char, String> {
    Grammar::Recursion(Rc::new(|ir| ir.clone()))
}

// This consumes a token before recursion, preventing stack overflow
pub fn infinite_shift1() -> Grammar<char, String> {
    use Grammar::*;
    Shift(Rc::new(move |c|
        infinite_shift1()
    ))
}
// This consumes a token before recursion, preventing stack overflow
pub fn infinite_shift2() -> Grammar<char, String> {
    use Grammar::*;
    Grammar::Recursion(Rc::new(|is| {
        let is_clone = is.clone();
        Shift(Rc::new(
            move |c| {
                let is_clone = is_clone.clone();
                is_clone
            }
        ))
    }))
}

// This consumes a token before recursion, preventing stack overflow
pub fn last1() -> Grammar<char, String> {
    use Grammar::*;
    item()
    .then(|c|
        last1()
        .or(Nonterminal(format!("{c}")))
    )
}
pub fn last2() -> Grammar<char, String> {
    use Grammar::*;
    Grammar::Recursion(Rc::new(|last| {
        let last_clone = last.clone();
        item()
        .then(move |c| {
            let last_clone = last_clone.clone();
            last_clone
            .or(Nonterminal(format!("{c}")))
        })
    }))
}

pub fn binary_string() -> Grammar<char, String> {
    use Grammar::*;
    Grammar::Recursion(Rc::new(|bs| {
        println!("bs = {bs:?}");
        let bs1 = bs.clone();
        let bs2 = bs1.clone();
        bs1
        .then(move |left: & String| {
            let left_clone = left.clone();
            bs2.clone()
            .then(move |right: & String| {
                let right_clone = right.clone();
                Nonterminal(format!("({left_clone} {right_clone})"))
            })
        })
        .or(item()
            .then(|c| {
                Nonterminal(format!("{c}"))
            })
        )
    }))
}

pub fn binary_string1() -> Grammar<char, String> {
    use Grammar::*;
    let recursive = |bs: & Grammar<char, String>| {
        let bs1 = bs.clone();
        let bs2 = bs1.clone();
        bs1
        .then(move |left: & String| {
            let left_clone = left.clone();
            bs2.clone()
            .then(move |right: & String| {
                let right_clone = right.clone();
                Nonterminal(format!("({left_clone} {right_clone})"))
            })
        })
        .or(item()
            .then(|c| {
                Nonterminal(format!("{c}"))
            })
        )
    };
    Recursion(Rc::new(recursive))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_infinite_shift() {
        let g = infinite_shift1();
        let x = g.shift(& 'a');
        assert_eq!(
            format!("{x:?}"),
            "Shift"
        );
        let g = infinite_shift2();
        let x = g.shift(& 'a');
        assert_eq!(
            format!("{x:?}"),
            "Recursion"
        )
    }

    #[test]
    fn test_last() {
        let g = last1();
        let x = g.shift(& 'a').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Shift, Nonterminal(\"a\")]"
        );
        let x = x[0].shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Shift, Nonterminal(\"b\")]"
        );
        let x = g.parse(& vec!['a', 'b', 'c', '1', '?', '!']);
        assert_eq!(
            format!("{x:?}"),
            "[Shift, Nonterminal(\"!\")]"
        );
        let g = last2();
        let x = g.shift(& 'a').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Recursion, Nonterminal(\"a\")]"
        );
        let x = x[0].shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Recursion, Nonterminal(\"b\")]"
        );
        let x = g.parse(& vec!['a', 'b', 'c', '1', '?', '!']);
        assert_eq!(
            format!("{x:?}"),
            "[Recursion, Nonterminal(\"!\")]"
        );
    }

    #[test]
    fn test_binary1() {
        let g = binary_string();
        println!("g = {g:?}");
        let x = g.shift(& 'a').reduce();
        // let x = g.parse(&"a".chars().collect::<Vec<char>>());
        assert_eq!(
            format!("{x:?}"),
            "[Shift, Nonterminal(\"a\")]"
        );
        let x = x[0].shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\"), Shift]"
        );
    }
    #[test]
    fn test_binary2() {
        let input = "ab".chars().collect();
        let x = binary_string().parse(&input);
        assert_eq!(
            format!("{x:?}"),
            "[Shift, Nonterminal(\"(a b)\")]"
        );
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
}
