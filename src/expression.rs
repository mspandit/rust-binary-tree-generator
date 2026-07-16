use crate::grammar::{Grammar};
use std::{fmt::Debug, rc::Rc};

pub fn item() -> Grammar<char, char> {
    Grammar::Shift(Rc::new(|input: &char| Grammar::Nonterminal(*input)))
}

fn sat(p: impl Fn(char) -> bool + 'static + Clone) -> Grammar<char, char> {
    item().then(move |c: & char|
        if p(* c) {
            Grammar::Nonterminal(*c)
        } else {
            Grammar::Reduce(vec![])
        }
    )
}

pub fn digit() -> Grammar<char, char> {
    sat(|c| c.is_ascii_digit())
}

fn character(c: char) -> Grammar<char, char> {
    sat(move |x| x == c)
}

fn letter() -> Grammar<char, char> {
    sat(|c| c.is_ascii_alphabetic())
}

fn nat() -> Grammar<char, i64> {
    digit().plus().then(|ds| {
        Grammar::Nonterminal(
            ds.into_iter()
            .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap() as i64)
        )
    })
}

fn space() -> Grammar<char, ()> {
    sat(|c| c.is_whitespace()).star().then(|_| Grammar::Nonterminal(()))
}

fn token<T: Clone + 'static + Debug>(p: Grammar<char, T>) -> Grammar<char, T> {
    space().then(move |_| {
        p.clone().then(move |res| {
            let res = res.clone();
            space().then(move |_| Grammar::Nonterminal(res.clone()))
        })
    })
}

fn int() -> Grammar<char, i64> {
    nat().or(character('-').then(|_| nat().then(|n| Grammar::Nonterminal(-n))))
}

pub fn integer() -> Grammar<char, i64> {
    token(int())
}

fn factor() -> Grammar<char, i64> {
    character('(')
        .then(move |_| {
            expr().then(move |x| {
                let x = x.clone();
                character(')').then(move |_| Grammar::Nonterminal(x.clone()))
            })
        })
        .or(integer())
}
fn term() -> Grammar<char, i64> {
    factor()
        .then(|x| {
            let x = x.clone();
            character('*').then(move |_| term().then(move |y| Grammar::Nonterminal(x * y)))
        })
        .or(factor())
}
fn expr() -> Grammar<char, i64> {
    term()
        .then(move |x| {
            let x = x.clone();
            character('+').then(move |_| expr().then(move |y| Grammar::Nonterminal(x + y)))
        })
        .or(term())
}

fn binop() -> Grammar<char, String> {
    character('-')
        .or(character('+'))
        .or(character('*'))
        .then(|c| Grammar::Nonterminal(c.to_string()))
}

fn ebo() -> Grammar<char, String> {
    expression().then(|_| binop())
}

pub fn expression() -> Grammar<char, String> {
    Grammar::Nonterminal("Unimplemented".to_string())
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_zero_characters() {
        let input = vec![];
        let x = expression().parse(&input);
        assert_eq!("[Shift]", format!("{:?}", x));
    }

    #[test]
    fn test_one_character() {
        let input = vec!['1'];
        let x = expression().parse(& input);
        assert_eq!("[Nonterminal(1), Shift]", format!("{:?}", x));
    }

    #[test]
    fn test_item() {
        let g = item();
        let x = g.parse(&vec!['a']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('a')));
    }

    #[test]
    fn test_two_characters() {
        let input = "-1".chars().collect();
        let x = expression().parse(& input);
        assert_eq!("[Nonterminal(-1), Cont(NDNary::Shift)]", format!("{:?}", x));
        let input = "1+".chars().collect();
        let x = expression().parse(& input);
        assert_eq!("[Nonterminal(1), Cont(NDNary::Shift)]", format!("{:?}", x));
    }

    #[test]
    fn test_three_characters() {
        let input = "1+3".chars().collect();
        let x = expression().parse(& input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal(4)]",
        )
    }

    #[test]
    fn test_four_characters() {
        let input = "-1+2*4".chars().collect();
        let x = expression().parse(& input);
        assert_eq!(5, x.len(), "{x:?}");
    }

    #[test]
    fn test_item_then() {
        let g = item().then(|c| {
            assert_eq!(*c, 'a');
            Grammar::Nonterminal("success")
        });
        let x = g.parse(&vec!['a']);
        assert_eq!(format!("{:?}", x), "[Nonterminal(\"success\")]",)
    }

    #[test]
    fn test_item_then_item_then1() {
        let g = item().then(|c1| {
            let c1 = c1.clone();
            item().then(move |c2| {
                assert_eq!(c1, 'a');
                assert_eq!(*c2, 'b');
                Grammar::Nonterminal("success")
            })
        });
        let x = g.parse(&vec!['a', 'b']);
        assert_eq!(format!("{:?}", x), "[Nonterminal(\"success\")]",)
    }

    #[test]
    fn test_item_then_item_then2() {
        let g = item()
            .then(|c1| {
                assert_eq!(*c1, 'a');
                item()
            })
            .then(|c2| {
                assert_eq!(*c2, 'b');
                Grammar::Nonterminal("success")
            });
        let x = g.parse(&vec!['a', 'b']);
        assert_eq!(format!("{:?}", x), "[Nonterminal(\"success\")]",)
    }

    #[test]
    fn test_sat() {
        let g = sat(|c| c == 'a');
        let x = g.parse(&vec!['b']);
        println!("{:?}", x);
        assert_eq!(x.len(), 0);
    }

    #[test]
    fn test_digit_parse() {
        assert_eq!(
            format!("{:?}", digit().parse(&vec!['1'])),
            "[Nonterminal('1')]"
        );
    }

    #[test]
    fn test_digit_parse_non_digit() {
        assert_eq!(format!("{:?}", digit().parse(&vec!['a', 'b', 'c'])), "[]");
    }
    #[test]
    fn test_character_parse() {
        assert_eq!(
            format!("{:?}", character('a').parse(&vec!['a'])),
            "[Nonterminal('a')]"
        );
    }
    #[test]
    fn test_character_parse_non_char() {
        assert_eq!(format!("{:?}", character('a').parse(&vec!['1'])), "[]");
    }
    #[test]
    fn test_digit_or_letter_parse() {
        assert_eq!(
            format!("{:?}", digit().or(letter()).parse(&vec!['a'])),
            "[Nonterminal('a')]"
        );
    }
    #[test]
    fn test_character_then_parse() {
        let g = character('a').then(|_| character('b').then(|_| Grammar::Nonterminal("success")));
        assert_eq!(
            format!("{:?}", g.parse(&vec!['a', 'b'])),
            "[Nonterminal(\"success\")]"
        );
    }
    #[test]
    fn test_character_or_parse() {
        let g = character('a').or(character('b')).then(|ab| Grammar::Nonterminal(format!("{ab}")));
        assert_eq!(
            format!("{:?}", g.clone().parse(&vec!['a'])),
            "[Nonterminal(\"a\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(&vec!['b'])),
            "[Nonterminal(\"b\")]"
        );
        assert_eq!(format!("{:?}", g.parse(&vec!['c'])), "[]");
    }
    #[test]
    fn test_character_or_parse1() {
        let ab = character('a').then(|_| character('b').then(|_| Grammar::Nonterminal(format!("ab"))));
        let ac = character('a').then(|_| character('c').then(|_| Grammar::Nonterminal(format!("ac"))));
        let g = ab.or(ac);
        assert_eq!(
            format!("{:?}", g.clone().parse(&vec!['a', 'b'])),
            "[Nonterminal(\"ab\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(&vec!['a', 'c'])),
            "[Nonterminal(\"ac\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(&vec!['a'])),
            "[Shift, Shift]"
        );
    }
    #[test]
    fn test_character_plus_parse0() {
        let x = character('a').plus().parse(&vec![]);
        assert_eq!(format!("{:?}", x), "[Shift]");
    }
    #[test]
    fn test_character_star_parse0() {
        let x = character('a').star().parse(&vec![]);
        assert_eq!(format!("{:?}", x), "[Shift, Nonterminal([])]");
    }
    #[test]
    fn test_character_plus_parse1() {
        let g = character('a').plus();
        let x = g.reduce();
        assert_eq!(format!("{:?}", x), "[Shift]");
        let x = x[0].shift(&'a');
        assert_eq!(format!("{:?}", x), "Reduce([Shift, Nonterminal(['a'])])");
        let x = g.shift(&'a');
        assert_eq!(format!("{:?}", x), "Reduce([Shift, Nonterminal(['a'])])");
        let x = g.parse(&vec!['a']);
        assert_eq!(format!("{:?}", x), "[Shift, Nonterminal(['a'])]");
    }
    #[test]
    fn test_digit_star_parse() {
        assert_eq!(
            format!("{:?}", digit().star().parse(&vec!['1', '2', '3'])),
            "[Shift, Nonterminal(['1', '2', '3'])]"
        );
    }
    #[test]
    fn test_digit_or_letter_star_parse() {
        assert_eq!(
            format!(
                "{:?}",
                digit()
                    .or(letter())
                    .star()
                    .parse(&vec!['a', 'b', 'c', '1', '2', '3'])
            ),
            "[Shift, Shift, Nonterminal(['a', 'b', 'c', '1', '2', '3'])]"
        );
    }
    #[test]
    fn test_nat_parse() {
        assert_eq!(
            format!("{:?}", nat().parse(&vec!['1', '2', '3',])),
            "[Shift, Nonterminal(123)]"
        );
    }
    #[test]
    fn test_integer_parse() {
        assert_eq!(
            format!("{:?}", integer().parse(&vec!['-', '4', '2',])),
            "[Shift, Shift, Nonterminal(-42)]"
        )
    }
    #[test]
    fn test_factor_parse() {
        assert_eq!(
            format!("{:?}", factor().parse(&vec!['(', '-', '4', '2', ')',])),
            "[Nonterminal(-42)]"
        );
    }
    #[test]
    fn test_factor_parse1() {
        assert_eq!(
            format!("{:?}", factor().parse(&vec!['-', '4', '2',])),
            "[Shift, Shift, Nonterminal(-42)]"
        );
    }
    #[test]
    fn test_term_parse() {
        let input = "3*4".chars().collect();
        let r: Vec<Grammar<char, i64>> = term()
            .parse(&input)
            .into_iter()
            .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
            .collect();
        assert_eq!(format!("{:?}", r), "[Nonterminal(12)]");
    }
    #[test]
    fn test_expr_parse() {
        let r: Vec<Grammar<char, i64>> = expr()
            .parse(&vec!['2', '+', '3', '*', '4'])
            .into_iter()
            .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
            .collect();
        assert_eq!(format!("{:?}", r), "[Nonterminal(14)]");
    }
    #[test]
    fn test_expr_parse_with_parentheses1() {
        let input: Vec<char> = "(2+3)*4".chars().collect();
        let r: Vec<Grammar<char, i64>> = expr()
            .parse(&input)
            .into_iter()
            .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
            .collect();
        assert_eq!(format!("{:?}", r), "[Nonterminal(20)]");
    }
    #[test]
    fn test_expr_parse_with_parentheses2() {
        let input: Vec<char> = "(2+(7*10)+8)*20".chars().collect();
        let r: Vec<Grammar<char, i64>> = expr()
            .parse(&input)
            .into_iter()
            .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
            .collect();
        assert_eq!(format!("{:?}", r), "[Nonterminal(1600)]");
    }

    #[test]
    fn test_expr_parse_fail1() {
        let input = "2+3*".chars().collect();
        let r: Vec<Grammar<char, i64>> = expr()
            .parse(&input)
            .into_iter()
            .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
            .collect();
        assert_eq!(format!("{:?}", r), "[]");
    }

    #[test]
    fn test_expr_parse_fail2() {
        let input = "(2+3".chars().collect();
        let r: Vec<Grammar<char, i64>> = expr()
            .parse(&input)
            .into_iter()
            .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
            .collect();
        assert_eq!(format!("{:?}", r), "[]");
    }
}
