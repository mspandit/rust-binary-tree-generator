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

    fn recurse3(src1: & Grammar<char, String>, src2: & Grammar<char, String>, src3: & Grammar<char, String>) -> Grammar<char, String> {
        use Grammar::*;
        let src3_clone = src3.clone();
        src1.or(
            & src2.then(move |left: & String| {
                let left_clone = left.clone();
                src3_clone.then(move |right: & String| {
                    Nonterminal(format!("({left_clone} {right})"))
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
    fn test_binary_from_scratch4() {
        use Grammar::*;
        // Successfully parses a single token, but
        // generates no continuation
        let g1: Grammar<char, String> = item();
        let x = g1.shift(& 'a').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"a\")]"
        );
        let g1_clone1 = g1.clone();
        let g1_clone2 = g1.clone();
        let g1_clone3 = g1.clone();
        // Successfully parses a single token, generating
        // a nonterminal and a continuation
        let g4 = recurse(& item());
        let x= g4.shift(& 'a').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"a\"), Shift]"
        );
        // Successfully parses a second token, but generates
        // no continuation.
        let x = x[1].shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\")]"
        );
        let g5 = recurse(& g1.shift(& 'a'));
        assert_eq!(
            format!("{g5:?}"),
            "Reduce([Nonterminal(\"a\"), Shift])"
        );
        let x = g5.shift(& 'b').reduce();
        assert_eq!(
            format!("{x:?}"),
            "[Nonterminal(\"(a b)\"), Shift]"
        );
        // Successfully parses two tokens, but generates
        // no continuation
        let g2: Grammar<char, String> = g1_clone1.then(move |left: & String| {
            let left_clone = left.clone();
            g1.then(move |right: & String| {
                Nonterminal(format!("({left_clone} {right})"))
            })
        });

        let g3: Grammar<char, String> = g1_clone2.then(move |left: & String| {
            let left_clone = left.clone();
            g1_clone3.then(move |right: & String| {
                Nonterminal(format!("({left_clone} {right})"))
            })
        });
    }

    #[test]
    fn test_binary_from_scratch5() {
        use Grammar::*;

        let base: Grammar<char, String> = item()
        .then(|c: & char| Nonterminal(format!("{c}")));

        // Note, the type of recursive is neither Grammar<char, String>
        // nor Grammar<String, String>
        let recursive = |base1: & Grammar<char, String>| {
            let base1_clone = base1.clone();
            move |base2: & Grammar<char, String>| {
                let base2_clone = base2.clone();
                base1_clone.then(move |left: & String| {
                    let left_clone = left.clone();
                    base2_clone.then(move |right: & String| {
                        Nonterminal(format!("({left_clone} {right})"))
                    })
                })
            }
        };
        let b0 = base.clone();
        let r0 = recursive.clone();

        let n10 = b0.shift(& 'a');
        let r10 = r0(& n10);
        let c11 = r10;

        let n20 = b0.shift(& 'b');
        let r20 = r0(& n20);

        let n21 = c11(& n20);
        let c21 = |s: & Grammar<char, String>| c11(& r20(s));
        let c22 = |s: & Grammar<char, String>| r0(& n21)(s);
        assert_eq!(
            format!("{:?}", n21.reduce()),
            "[Nonterminal(\"(a b)\")]"
        );

        let n30 = b0.shift(& 'c');
        let r30 = r0(& n30);

        let n31 = c21(& n30);
        assert_eq!(
            format!("{:?}", n31.reduce()),
            "[Nonterminal(\"(a (b c))\")]"
        );

        let n32 = c22(& n30);
        assert_eq!(
            format!("{:?}", n32.reduce()),
            "[Nonterminal(\"((a b) c)\")]"
        );
        let r31 = r0(& n31);
        let r32 = r0(& n32);

        let c31 = |s: & Grammar<char, String>| c21(& r30(s));
        let c32 = |s: & Grammar<char, String>| c21(& r20(& r30(s)));
        let c33 = |s: & Grammar<char, String>| c22(& r30(s));
        let c34 = |s: & Grammar<char, String>| r0(& n31)(s);
        let c35 = |s: & Grammar<char, String>| r0(& n32)(s);

        let n40 = b0.shift(& 'd');
        let r40 = r0(& n40);

        let n41 = c31(& n40);
        assert_eq!(
            format!("{:?}", n41.reduce()),
            "[Nonterminal(\"(a (b (c d)))\")]"
        );

        let n42 = c32(& n40);
        assert_eq!(
            format!("{:?}", n42.reduce()),
            "[Nonterminal(\"(a ((b c) d))\")]"
        );

        let n43 = c33(& n40);
        assert_eq!(
            format!("{:?}", n43.reduce()),
            "[Nonterminal(\"((a b) (c d))\")]"
        );

        let n44 = c34(& n40);
        assert_eq!(
            format!("{:?}", n44.reduce()),
            "[Nonterminal(\"((a (b c)) d)\")]"
        );

        let n45 = c35(& n40);
        assert_eq!(
            format!("{:?}", n45.reduce()),
            "[Nonterminal(\"(((a b) c) d)\")]"
        )
    }

    #[test]
    fn test_binary_from_scratch6() {
        use Grammar::*;
        let b: Grammar<char, String> = item().then(move |c: & char| Nonterminal(format!("{c}")));
        let g1 = recurse3(&b, &b, &b);
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

        let g2 = recurse3(& x, & x, & b);
        let x = g2.shift(& 'b');
        assert_eq!(
            format!("{:?}", x.reduce()),
            "[Nonterminal(\"(a b)\"), Nonterminal(\"(a b)\"), Shift]"
        );
        let g3 = recurse3(& x, & x, &b);
        let x = g3.shift(& 'c');
        assert_eq!(
            format!("{:?}", x.reduce()),
            "[Nonterminal(\"(a b)\"), Shift]"
        );
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
