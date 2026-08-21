use std::rc::Rc;

use crate::grammar::{Grammar, item, left_recursive};

// This overflows the stack during grammar definition
pub fn _infinite_reduce1() -> Grammar<char, String> {
    Grammar::Reduce(vec![_infinite_reduce1()])
}

// This consumes a token before recursion, preventing stack overflow
pub fn _infinite_shift1() -> Grammar<char, String> {
    use Grammar::*;
    Shift(Rc::new(move |_c|
        _infinite_shift1()
    ))
}

// (Recursive) generator returns a binary string
// grammar that consumes $n$ tokens.
// S <- char
// S <- S S
fn binary_generator(n: usize) -> Grammar<char, String> {
    use Grammar::*;
    match n {
        0 => Reduce(vec![]),
        1 => item(),
        n => (1..n).fold(
            binary_generator(0),
            |g, i| g.or(& binary_generator(i)
                .then(move |left: & String| {
                    let left_clone = left.clone();
                    binary_generator(n - i).then(move |right: & String| {
                        Nonterminal(format!("({left_clone} {right})"))
                    })
                })
            )
        ),
    }
}

pub fn binary_string() -> Grammar<char, String> {
    left_recursive(binary_generator)
}

#[cfg(test)]
mod test {

    use super::*;

    // This consumes a token before recursion, preventing stack overflow
    pub fn last1() -> Grammar<char, String> {
        use Grammar::*;
        item()
        .then(|c: & char|
            last1()
            .or(& Nonterminal(format!("{c}")))
        )
    }

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
            "[Nonterminal(\"a\")]"
        );
        let x = g.parse(&vec!['a', 'b']);
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\")]"
        );
    }

    fn recurse2(src1: & Grammar<char, String>, src2: & Grammar<char, String>) -> Grammar<char, String> {
        use Grammar::*;
        let src2_clone = src2.clone();
        src1.then(move |left: & String| {
            let left_clone = left.clone();
            src2_clone.then(move |right: & String| {
                Nonterminal(format!("({left_clone} {right})"))
            })
        })
    }

    #[test]
    fn test_binary_from_scratch7() {
        use Grammar::*;
        let b: Grammar<char, String> = item().then(move |c: & char| Nonterminal(format!("{c}")));
        // g1.shift(& 'a') -> "a", "(a ?)"
        let g1 = b.clone().or(&recurse2(&b, &b));
        let x = g1.shift(& 'a');
        assert_eq!(
            format!("{:?}", x.reduce()),
            "[Nonterminal(\"a\"), Shift]"
        );
        let x1 = x.reduce()[1].shift(& 'b');
        assert_eq!(
            format!("{:?}", x1.reduce()),
            "[Nonterminal(\"(a b)\")]"
        );
        // [a, (a ?)] then [?, (? ?)] = [(a ?), (a (? ?)), ((a ?) ?), ((a ?) (? ?))]
        // g2.shift(& 'b') -> "(a b)", "(a (b ?))", "((a b) ?)"
        let g2: Grammar<char, String> = x.or(& recurse2(& x, & g1));
        let x = g2.shift(& 'b');
        assert_eq!(
            format!("{:?}", x.reduce()),
            // Undesirable repetition
            "[Nonterminal(\"(a b)\"), Nonterminal(\"(a b)\"), Shift, Shift, Shift]" // (a b) (a (b ?)) ((a b) ?)
        );
        let x1 = x.reduce()[2].shift(& 'c');
        assert_eq!(
            format!("{:?}", x1.reduce()),
            "[Nonterminal(\"(a (b c))\")]"
        );
        let x2 = x.reduce()[3].shift(& 'c');
        assert_eq!(
            format!("{:?}", x2.reduce()),
            "[Nonterminal(\"((a b) c)\")]"
        );
        let x3 = x.reduce()[4].shift(& 'c').shift(& 'd');
        assert_eq!(
            format!("{:?}", x3.reduce()),
            "[Nonterminal(\"((a b) (c d))\")]"
        );
        // [(a b), (a (b ?)), ((a b) ?), ((a b) (? ?))] then [?, (? ?)] =
        // [((a b) ?), ((a b) (? ?)), ((a (b ?)) ?), ((a (b ?)) (? ?)), (((a b) ?) ?), (((a b) ?) (? ?)), (((a b) (? ?)) ?), (((a b) (? ?)) (? ?))]
        // g3.shift(& 'c') -> "(a (b c))", "((a b) c)", Shift, Shift, Shift, Shift, Shift
        let g3: Grammar<char, String> = x.or(& recurse2(& x, & g1));
        let x = g3.shift(& 'c');
        assert_eq!(
            format!("{:?}", x.reduce()),
            // Undesirable repetition
            "[Nonterminal(\"(a (b c))\"), Nonterminal(\"((a b) c)\"), Shift, Nonterminal(\"((a b) c)\"), Shift, Nonterminal(\"((a b) c)\"), Shift, Shift, Shift, Shift, Shift, Shift]"
        );
        let x1 = x.reduce()[2].shift(& 'd');
        assert_eq!(
            format!("{:?}", x1.reduce()),
            "[Nonterminal(\"((a b) (c d))\")]"
        );
        let x2 = x.reduce()[4].shift(& 'd');
        assert_eq!(
            format!("{:?}", x2.reduce()),
            // Undesirable repetition
            "[Nonterminal(\"((a b) (c d))\")]"
        );
        let x3 = x.reduce()[6].shift(& 'd');
        assert_eq!(
            format!("{:?}", x3.reduce()),
            // Undesirable repetition
            "[Nonterminal(\"((a b) (c d))\")]"
        );
        let x4 = x.reduce()[7].shift(& 'd');
        assert_eq!(
            format!("{:?}", x4.reduce()),
            "[Nonterminal(\"((a (b c)) d)\")]"
        );
        let x5 = x.reduce()[8].shift(& 'd');
        assert_eq!(
            format!("{:?}", x5.reduce()),
            "[Shift]"
        );
        let x6 = x.reduce()[9].shift(& 'd');
        assert_eq!(
            format!("{:?}", x6.reduce()),
            "[Nonterminal(\"(((a b) c) d)\")]"
        );
        let x7 = x.reduce()[10].shift(& 'd');
        assert_eq!(
            format!("{:?}", x7.reduce()),
            // Unacceptable missing combinations
            "[Shift]"
        );
        let x8 = x.reduce()[11].shift(& 'd');
        assert_eq!(
            format!("{:?}", x8.reduce()),
            // Unacceptable missing combinations
            "[Shift, Shift]"
        );
    }

    #[test]
    fn test_binary_string() {
        let g = binary_string();
        assert_eq!(
            format!("{:?}", g.shift(& 'a').reduce()),
            "[Shift, Nonterminal(\"a\")]"
        );
        assert_eq!(
            format!("{:?}", g.shift(& 'a').shift(& 'b').reduce()),
            "[Shift, Nonterminal(\"(a b)\")]"
        );
        assert_eq!(
            format!("{:?}", g.shift(& 'a').shift(& 'b').shift(& 'c').reduce()),
            "[Shift, Nonterminal(\"(a (b c))\"), Nonterminal(\"((a b) c)\")]"
        );
        assert_eq!(
            format!("{:?}", g.shift(& 'a').shift(& 'b').shift(& 'c').shift(& 'd').reduce()),
            "[Shift, Nonterminal(\"(a (b (c d)))\"), Nonterminal(\"(a ((b c) d))\"), Nonterminal(\"((a b) (c d))\"), Nonterminal(\"((a (b c)) d)\"), Nonterminal(\"(((a b) c) d)\")]"
        );
        assert_eq!(
            g.shift(& 'a').shift(& 'b').shift(& 'c').shift(& 'd').shift(& 'e').shift(& 'f').reduce().len(),
            43
        )
    }

    #[test]
    fn test_binary_from_scratch8() {
        let g1 = binary_generator(1);
        assert_eq!(
            format!("{:?}", g1.shift(& 'a').reduce()),
            "[Nonterminal(\"a\")]"
        );

        let g2 = binary_generator(2).shift(& 'a');
        assert_eq!(
            format!("{:?}", g2.shift(& 'b').reduce()),
            "[Nonterminal(\"(a b)\")]"
        );

        let g3 = binary_generator(3).shift(& 'a').shift(& 'b');
        assert_eq!(
            format!("{:?}", g3.shift(& 'c').reduce()),
            "[Nonterminal(\"(a (b c))\"), Nonterminal(\"((a b) c)\")]"
        );

        let g4 = binary_generator(4).shift(& 'a').shift(& 'b').shift(& 'c');
        assert_eq!(
            format!("{:?}", g4.shift(& 'd').reduce()),
            "[Nonterminal(\"(a (b (c d)))\"), Nonterminal(\"(a ((b c) d))\"), Nonterminal(\"((a b) (c d))\"), Nonterminal(\"((a (b c)) d)\"), Nonterminal(\"(((a b) c) d)\")]"
        )
    }

    #[test]
    fn test_infinite_shift() {
        let g = _infinite_shift1();
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
            "[Nonterminal(\"!\")]"
        );
    }
}
