use std::{rc::Rc, fmt::Debug};
use crate::grammar::{Grammar, recursive};


fn item() -> Grammar<char, char> {
    Grammar::Shift(Rc::new(|input: &char| {
        vec![Grammar::Nonterminal(*input)]
    }))
}

fn sat(p: impl Fn(char) -> bool + 'static + Clone) -> Grammar<char, char> {
    Grammar::Shift(Rc::new(move |input: &char| {
        if p(*input) {
            vec![Grammar::Nonterminal(*input)]
        } else {
            vec![]
        }
    }))
}

pub fn digit() -> Grammar<char, char> {
    sat(|c| {
        c.is_ascii_digit()
    })
}

fn character(c: char) -> Grammar<char, char> {
    sat(move |x| x == c)
}

fn character1() -> Grammar<char, char> {
    sat(move |x|
        x == '1'
    )
}

fn letter() -> Grammar<char, char> {
    sat(|c| {
        c.is_ascii_alphabetic()
    })
}

fn nat() -> Grammar<char, i64> {
    digit()
    .plus()
    .then(|ds| ds.into_iter()
        .fold(0, |acc, d|
            acc * 10 + d.to_digit(10).unwrap() as i64
        ).into()
    )
}

fn space() -> Grammar<char, ()> {
    sat(|c| c.is_whitespace()).star().then(|_| ().into())
}

fn token<T: Clone + 'static + Debug>(p: Grammar<char, T>) -> Grammar<char, T> {
    space().then(move |_| {
        p.clone().then(move |res| {
            space().then(move |_| {
                res.clone().into()
            })
        })
    })
}

fn int() -> Grammar<char, i64> {
    nat().or(
        character('-').then(|_| {
            nat().then(|n| {
                (-n).into()
            })
        })
    )
}

pub fn integer() -> Grammar<char, i64> {
    token(int())
}

fn factor() -> Grammar<char, i64> {
    character('(').then(move |_| {
        expr().then(|x| {
            character(')').then(move |_| {
                x.into()
            })
        })
    }).or(integer())
}
fn term() -> Grammar<char, i64> {
    factor().then(|x| {
        character('*').then(move |_| {
            term().then(move |y| {
                (x * y).into()
            })
        })
    }).or(factor())
}
fn expr() -> Grammar<char, i64> {
    term().then(move |x| {
        character('+').then(move |_| {
            expr().then(move |y| {
                (x + y).into()
            })
        })
    }).or(term())
}

fn binop() -> Grammar<char, String> {
    character('-')
    .or(character('+'))
    .or(character('*'))
    .then(|c| c.to_string().into())
}

fn ebo() -> Grammar<char, String> {
    expression().then(|x| binop())
}

pub fn expression() -> Grammar<char, String> {
    recursive(&|expression: &Grammar<char, String>, x: &char|  {
        let ebo = expression.clone().then(|x| binop());
        let num = character1()
        .or(character('2'))
        .or(character('3'))
        .or(character('4')).then(|c| Grammar::from(c.to_string()));
        let unop = character('-');
        let e1 = expression.clone();
        let e2 = expression.clone();
        let r = num
        .or(unop
            .then(move |_c| e1.clone())
            .or(ebo
                .then(move |_s| e2.clone())
        ));
        r.apply(x)
    })
}
#[cfg(test)]
mod test {

use super::*;
    #[test]
    fn test_zero_characters() {
        let input = vec![];
        let x = expression()
        .parse(& input);
        assert_eq!("[Shift]", format!("{:?}", x));
    }

    // #[test]
    // fn test_one_character() {
    //     let input = vec!['1'];
    //     let x = expression().parse(& input);
    //     assert_eq!("[Nonterminal(1), Cont(Grammar::Shift)]", format!("{:?}", x));
    // }

    #[test]
    fn test_item() {
        let g = item();
        let x = g.parse(&vec!['a']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('a')));
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
    fn test_item_then() {
        let g = item().then(|c| {
            assert_eq!(c, 'a');
            Grammar::from("success")
        });
        let x = g.parse(&vec!['a']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal("success")));
    }

    #[test]
    fn test_item_then_item_then1() {
        let g = item()
        .then(|c1| {
            item()
            .then(move |c2| {
                assert_eq!(c1, 'a');
                assert_eq!(c2, 'b');
                Grammar::from("success")
            })
        });
        let x = g.parse(&vec!['a', 'b']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal("success")));
    }

    #[test]
    fn test_item_then_item_then2() {
        let g = item()
        .then(|c1| {
            assert_eq!(c1, 'a');
            item()
        })
        .then(|c2| {
            assert_eq!(c2, 'b');
            Grammar::from("success")
        });
        let x = g.parse(&vec!['a', 'b']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal("success")));
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
        assert_eq!(
            format!("{:?}", digit().parse(&vec!['a', 'b', 'c'])),
            "[]"
        );
    }
    #[test]
    fn test_character_parse() {
        assert_eq!(
            format!("{:?}", character('a').parse(&vec!['a'])),
            "[Nonterminal('a')]"
        );
    }
    #[test]
    fn test_recursive_char_parse() {
        let r = recursive(&|_: &Grammar<char, char>, x| character('a').apply(x));
        assert_eq!(
            format!("{:?}", r.parse(&vec!['a'])),
            "[Nonterminal('a')]"
        )
    }
    #[test]
    fn test_character_parse_non_char() {
        assert_eq!(
            format!("{:?}", character('a').parse(&vec!['1'])),
            "[]"
        );
    }
    #[test]
    fn test_recursive_char_parse_non_char() {
        let r = recursive(&|_: &Grammar<char, char>, x| character('a').apply(x));
        assert_eq!(
            format!("{:?}", r.parse(&vec!['1'])),
            "[]"
        )
    }
    #[test]
    fn test_digit_or_letter_parse() {
        assert_eq!(
            format!("{:?}", digit().or(letter()).parse(& vec!['a'])),
            "[Nonterminal('a')]"
        );
    }
    #[test]
    fn test_character_then_parse() {
        let g = character('a')
        .then(|_| character('b')
            .then(|_| Grammar::from("success"))
        );
        assert_eq!(
            format!("{:?}", g.parse(& vec!['a', 'b'])),
            "[Nonterminal(\"success\")]"
        );
    }
    #[test]
    fn test_character_or_parse() {
        let g = character('a').or(character('b')).then(|ab| Grammar::from(format!("{ab}")));
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['a'])),
            "[Nonterminal(\"a\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['b'])),
            "[Nonterminal(\"b\")]"
        );
        assert_eq!(
            format!("{:?}", g.parse(& vec!['c'])),
            "[]"
        );
    }
    #[test]
    fn test_recursive_character_or_parse() {
        let g = recursive(&|_, x| character('a').or(character('b')).then(|ab| Grammar::from(format!("{ab}"))).apply(x));
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['a'])),
            "[Nonterminal(\"a\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['b'])),
            "[Nonterminal(\"b\")]"
        );
        assert_eq!(
            format!("{:?}", g.parse(& vec!['c'])),
            "[]"
        );
    }
    #[test]
    fn test_character_or_parse1() {
        let ab = character('a').then(|_| character('b').then(|_| Grammar::from(format!("ab"))));
        let ac = character('a').then(|_| character('c').then(|_| Grammar::from(format!("ac"))));
        let g = ab.or(ac);
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['a', 'b'])),
            "[Nonterminal(\"ab\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['a', 'c'])),
            "[Nonterminal(\"ac\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['a'])),
            "[Shift, Shift]"
        );
    }
    #[test]
    fn test_recursive_character_or_parse1() {
        let g = recursive(&|_: &Grammar<char, String>, x| {
            let ab = character('a').then(|_| character('b').then(|_| Grammar::from(format!("ab"))));
            let ac = character('a').then(|_| character('c').then(|_| Grammar::from(format!("ac"))));
            let ac2 = ac.clone();
            ab.clone().or(ac2.clone()).apply(x)
        });
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['a', 'b'])),
            "[Nonterminal(\"ab\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['a', 'c'])),
            "[Nonterminal(\"ac\")]"
        );
        assert_eq!(
            format!("{:?}", g.clone().parse(& vec!['a'])),
            "[Shift, Shift]"
        );
    }
    #[test]
    fn test_character_plus_parse0() {
        let x = character('a')
        .plus()
        .parse(& vec![]);
        assert_eq!(
            format!("{:?}", x),
            "[Shift]"
        );
    }
    #[test]
    fn test_character_star_parse0() {
        let x = character('a')
        .star()
        .parse(& vec![]);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal([]), Shift]"
        );
    }
    #[test]
    fn test_character_plus_parse1() {
        let x = character('a').plus().parse(& vec!['a']);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal(['a']), Shift]"
        );
    }
    #[test]
    fn test_digit_star_parse() {
        assert_eq!(
            format!("{:?}", digit().star().parse(& vec!['1', '2', '3'])),
            "[Nonterminal(['1', '2', '3']), Shift]"
        );
    }
   #[test]
    fn test_digit_or_letter_star_parse() {
        assert_eq!(
            format!("{:?}", digit().or(letter()).star().parse(& vec!['a', 'b', 'c', '1', '2', '3'])),
            "[Nonterminal(['a', 'b', 'c', '1', '2', '3']), Shift]"
        );
    }
    #[test]
    fn test_nat_parse() {
        assert_eq!(
            format!("{:?}", nat().parse(& vec!['1', '2', '3', ])),
            "[Nonterminal(123), Shift]"
        );
    }
   #[test]
    fn test_integer_parse() {
        assert_eq!(
            format!("{:?}", integer().parse(& vec!['-', '4', '2', ])),
            "[Nonterminal(-42), Shift, Shift]"
        )
    }
    #[test]
    fn test_factor_parse() {
        assert_eq!(
            format!("{:?}", factor().parse(& vec!['(', '-', '4', '2', ')', ])),
            "[Nonterminal(-42)]"
        );
    }
    #[test]
    fn test_factor_parse1() {
        assert_eq!(
            format!("{:?}", factor().parse(& vec!['-', '4', '2', ])),
            "[Nonterminal(-42), Shift, Shift]"
        );
    }
    #[test]
    fn test_term_parse() {
        let input = "3*4".chars().collect();
        let r: Vec<Grammar<char, i64>> = term().parse(& input)
            .into_iter().filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
            .collect();
        assert_eq!(
            format!("{:?}", r),
            "[Nonterminal(12)]"
        );
    }
    #[test]
    fn test_expr_parse() {
        let r: Vec<Grammar<char, i64>> = expr()
            .parse(& vec!['2', '+', '3', '*', '4', ])
            .into_iter().filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
            .collect();
        assert_eq!(format!("{:?}", r), "[Nonterminal(14)]");
    }
    #[test]
    fn test_expr_parse_with_parentheses1() {
        let input: Vec<char> = "(2+3)*4".chars().collect();
        let r: Vec<Grammar<char, i64>> = expr()
        .parse(& input)
        .into_iter()
        .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
        .collect();
        assert_eq!(
            format!("{:?}", r),
            "[Nonterminal(20)]"
        );
    }
    #[test]
    fn test_expr_parse_with_parentheses2() {
        let input: Vec<char> = "(2+(7*10)+8)*20".chars().collect();
        let r: Vec<Grammar<char, i64>> = expr()
        .parse(& input)
        .into_iter()
        .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
        .collect();
        assert_eq!(
            format!("{:?}", r),
            "[Nonterminal(1600)]"
        );
    }

    #[test]
    fn test_expr_parse_fail1() {
        let input = "2+3*".chars().collect();
        let r: Vec<Grammar<char, i64>> = expr()
        .parse(& input)
        .into_iter()
        .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
        .collect();
        assert_eq!(
            format!("{:?}", r),
            "[]"
        );
    }

    #[test]
    fn test_expr_parse_fail2() {
        let input = "(2+3".chars().collect();
        let r: Vec<Grammar<char, i64>> = expr().parse(& input)
        .into_iter()
        .filter(|pr| matches!(pr, Grammar::Nonterminal(_)))
        .collect();
        assert_eq!(
            format!("{:?}", r),
            "[]"
        );
    }
}
