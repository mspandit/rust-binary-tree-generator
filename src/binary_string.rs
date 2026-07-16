use std::rc::Rc;

use crate::{grammar::{Grammar, item}};

// This overflows the stack during grammar definition
pub fn infinite_reduce1() -> Grammar<char, String> {
    Grammar::Reduce(vec![infinite_reduce1()])
}

// This consumes a token before recursion, preventing stack overflow
pub fn infinite_shift1() -> Grammar<char, String> {
    use Grammar::*;
    Shift(Rc::new(move |c|
        infinite_shift1()
    ))
}

// This consumes a token before recursion, preventing stack overflow
pub fn last1() -> Grammar<char, String> {
    use Grammar::*;
    item()
    .then(|c: & char|
        last1()
        .or(& Nonterminal(format!("{c}")))
    )
}

#[cfg(test)]
mod test {

use super::*;

    #[test]
    fn test_binary_from_scratch1() {
        use Grammar::*;
        let c1: Grammar<char, String> = Shift(Rc::new(move |t| Nonterminal(format!("{t}"))));
        let c1_clone = c1.clone();
        let c2 = c1.clone()
        .then(move |lc| {
            let lc_clone = lc.clone();
            let c1_clone = c1.clone();
            c1_clone
            .then(move |rc|
                Nonterminal(format!("({lc_clone} {rc})"))
            )
        });
        let g = c1_clone.or(& c2);
        let x = g.parse(&vec!['a']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"a\"), Shift]"
        );
        let x = g.parse(&vec!['a', 'b']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\")]"
        );
    }

    #[test]
    fn test_binary_from_scratch2() {
        use Grammar::*;
        let base: Grammar<char, String> = Shift(Rc::new(move |t| Nonterminal(format!("{t}"))));
        let recursive: Grammar<String, String> = Shift(Rc::new(move |left| {
            let left_clone = left.clone();
            Shift(Rc::new(move |right| {
                Nonterminal(format!("({left_clone} {right}"))
            }))
        }));
    }

    fn recurse(src: & Grammar<char, String>) -> Grammar<char, String> {
        use Grammar::*;
        let src_clone0 = src.clone();
        let src_clone1 = src.clone();
        let src_clone2 = src.clone();
        src_clone0.or(
            & src_clone1.then(move |left: & String| {
                let left_clone = left.clone();
                src_clone2.then(move |right: & String| {
                    let right_clone = right.clone();
                    Nonterminal(format!("({left_clone} {right_clone})"))
                })
            })
        )
    }

    #[test]
    fn test_binary_from_scratch3() {
        use Grammar::*;
        // base case can consume 1 token
        let x: Vec<Grammar<char, String>> = item().shift(& 'a').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"a\")]"
        );
        // single recurse can consume 1 token
        let x: Vec<Grammar<char, String>> = recurse(& item()).shift(& 'a').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"a\"), Shift]"
        );
        // continuation can consume 2nd token
        let x = x[1].shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\")]"
        );
        // single recurse can consume 2 tokens
        let x = recurse(& item()).shift(& 'a').shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\")]"
        );
        // single recurse fails on 3rd token
        let x: Vec<Grammar<char, String>> = recurse(& item()).shift(& 'a').shift(& 'b').shift(& 'c').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[]"
        );
        // double recurse can consume 1 token
        let x: Vec<Grammar<char, String>> = recurse(& recurse(& item())).shift(& 'a').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"a\"), Shift, Shift, Shift, Shift]"
        );
        // double recurse can consume 2nd token
        let x = recurse(& recurse(& item())).shift(& 'a').shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            // TODO: Eliminate this duplication
            "[Nonterminal(\"(a b)\"), Nonterminal(\"(a b)\"), Shift, Shift, Shift]"
        );
        // double recurse can consume 3rd token
        let x = recurse(& recurse(& item())).shift(& 'a').shift(& 'b').shift(& 'c').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a (b c))\"), Nonterminal(\"((a b) c)\"), Shift]"
        );
        let x = recurse(& recurse(& item())).shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\"), Shift]"
        );
        let x = recurse(& x[0]).shift(& 'c').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"((a b) c)\"), [Nonterminal(\"(a (b c))\"), Shift]"
        );
        let x = recurse(& item()).parse(& vec!['a', 'b']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\")]"
        );
        let x = recurse(& recurse(& item())).parse(& vec!['a', 'b']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\"), Shift, Shift, Shift]"
        );
        let x = recurse(& recurse(& item())).parse(& vec!['a', 'b', 'c']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a (b c))\"), Nonterminal(\"((a b) c)\"), Shift]"
        );
        let x = recurse(& recurse(& recurse(& item()))).parse(& vec!['a', 'b', 'c']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a (b c))\"), Shift, Shift, Shift, Nonterminal(\"((a b) c)\"), Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift]"
        );
        let x = recurse(& recurse(& recurse(& item()))).parse(& vec!['a', 'b', 'c', 'd']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a (b (c d)))\"), Nonterminal(\"(a ((b c) d))\"), Shift, Nonterminal(\"((a b) (c d))\"), Shift, Shift, Shift, Nonterminal(\"((a (b c)) d)\"), Shift, Shift, Shift, Nonterminal(\"(((a b) c) d)\"), Shift, Shift, Shift, Shift, Shift, Shift]"
        );
        let x = recurse(& recurse(& recurse(& recurse(& item())))).parse(& vec!['a', 'b', 'c', 'd']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a (b (c d)))\"), Shift, Shift, Shift, Nonterminal(\"(a ((b c) d))\"), Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Nonterminal(\"((a b) (c d))\"), Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Nonterminal(\"((a (b c)) d)\"), Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Nonterminal(\"(((a b) c) d)\"), Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift, Shift]"
        );
    }

    #[test]
    fn test_double_recurse_fails_on_4th_token() {
        // double recurse should fail on 4th token
        let x = recurse(& recurse(& item())).shift(& 'a').shift(& 'b').shift(& 'c').shift(& 'd').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[]"
        );

    }

    #[test]
    fn test_infinite_shift() {
        let g = infinite_shift1();
        let x = g.shift(& 'a');
        assert_eq!(
            format!("{x:?}"),
            "Shift"
        );
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
    }
}
