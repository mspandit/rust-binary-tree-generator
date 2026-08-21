use crate::grammar::{Grammar, item, left_recursive};
use std::{fmt::Debug};

fn sat(p: impl Fn(char) -> bool + 'static + Clone) -> Grammar<char, char> {
    item::<char, char>().then(move |c: &char| {
        if p(*c) {
            Grammar::Nonterminal(*c)
        } else {
            Grammar::Reduce(vec![])
        }
    })
}

fn character(c: char) -> Grammar<char, char> {
    sat(move |x| x == c)
}

#[derive(Clone)]
pub enum Expression {
    UnOp(String),
    E(String),
    BinOp(String),
}

impl Debug for Expression {
    fn fmt(self: & Self, f: & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        match self {
            UnOp(s) | E(s) | BinOp(s) => write!(f, "{}", s)
        }
    }
}

// Number <- '1' | '2' | '3' | '4'
fn number() ->Grammar<char, Expression> {
    use Grammar::*;
    character('1')
    .or(& character('2'))
    .or(& character('3'))
    .or(& character('4'))
    .then(move |c| {
        Nonterminal(Expression::E(format!("{c}")))
    })
}

// UnOp <- '-' | '+'
fn un_op() -> Grammar<char, Expression> {
    use Grammar::*;
    character('-')
    .or(& character('+'))
    .then(move |c| {
        Nonterminal(Expression::UnOp(format!("{c}")))
    })
}

// BinOp <- '*' | '-' | '+'
fn bin_op() -> Grammar<char, Expression> {
    use Grammar::*;
    character('-')
    .or(& character('+'))
    .or(& character('*'))
    .then(move |c| {
        Nonterminal(Expression::BinOp(format!("{c}")))
    })
}

// (Recursive) generator returns an expression
// grammar that consumes $n$ tokens.
// E <- Number
// E <- UnOp E | E BinOp E
fn expr_gen(n: usize) -> Grammar<char, Expression> {
    use Grammar::*;
    match n {
        0 => Reduce(vec![]),
        1 => number(),
        n => (1..n).fold(
            expr_gen(0),
            |res, i| res.or(& expr_gen(i)
                .then(move |left_e| {
                    let le2 = left_e.clone();
                    bin_op().then(move |op| {
                        let op_clone = op.clone();
                        let le2 = le2.clone();
                        expr_gen(n - i).then(move |right_e| {
                            use Expression::*;
                            let re2 = right_e.clone();
                            let le2 = le2.clone();
                            Nonterminal(
                                E(format!("({le2:?} {op_clone:?} {re2:?})"))
                            )
                        })
                    })
                })
            )
        )
        .or(
            & un_op().then(move |op| {
                let op_clone = op.clone();
                expr_gen(n - 1).then(move |e | {
                    use Expression::*;
                    let e_clone = e.clone();
                    let op_clone = op_clone.clone();
                    match (op_clone, e_clone) {
                        (UnOp(op), E(e)) => Nonterminal(E(format!("{op}{e}"))),
                        _ => panic!("Unexpected pattern"),
                    }
                })
            })
        )
    }
}

pub fn expression() -> Grammar<char, Expression> {
    left_recursive(expr_gen)
}

#[cfg(test)]
mod test {

    use std::rc::Rc;
    use super::*;

    fn letter() -> Grammar<char, char> {
        sat(|c| c.is_ascii_alphabetic())
    }

    pub fn digit() -> Grammar<char, char> {
        sat(|c| c.is_ascii_digit())
    }

    fn nat() -> Grammar<char, i64> {
        digit().plus().then(|ds| {
            Grammar::Nonterminal(
                ds.into_iter()
                    .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap() as i64),
            )
        })
    }

    fn space() -> Grammar<char, ()> {
        sat(|c| c.is_whitespace())
            .star()
            .then(|_| Grammar::Nonterminal(()))
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
        nat().or(&character('-').then(|_| nat().then(|n| Grammar::Nonterminal(-n))))
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
            .or(&integer())
    }
    fn term() -> Grammar<char, i64> {
        factor()
            .then(|x| {
                let x = x.clone();
                character('*').then(move |_| term().then(move |y| Grammar::Nonterminal(x * y)))
            })
            .or(&factor())
    }
    pub fn expr() -> Grammar<char, i64> {
        term()
            .then(move |x| {
                let x = x.clone();
                character('+').then(move |_| expr().then(move |y| Grammar::Nonterminal(x + y)))
            })
            .or(&term())
    }

    fn binop() -> Grammar<char, char> {
        character('+')
        .or(&character('*'))
        .then(|c| Grammar::Nonterminal(*c))
    }

    fn unop() -> Grammar<char, i64> {
        character('-').then(|_c| Grammar::Nonterminal(0))
    }

    pub fn expr_item() -> Grammar<char, i64> {
        use Grammar::*;
        Shift(Rc::new(move |c| c.to_digit(10).map_or(Reduce(vec![]), |d| Nonterminal(d as i64))))
    }

    fn express_gen(src: & Grammar<char, i64>) -> Grammar<char, i64> {
        use Grammar::*;
        let src_clone0 = src.clone();
        let src_clone1 = src.clone();
        let src_clone2 = src.clone();
        let ebo = src_clone2.then(move |e1| {
            let e1_clone = e1.clone();
            binop().then(move |bo| {
                Nonterminal((e1_clone, *bo))
            })
        });
        expr_item()
        .or(
            & unop().then(move |_unop| {
                src_clone0.then(move |e| {
                    Nonterminal(-e)
                })
            })
        )
        .or(
            & ebo.then(move |ebo| {
                let ebo_clone = ebo.clone();
                src_clone1.then(move |e2| {
                    match ebo_clone {
                        (e1, '+') => Nonterminal(e1 + *e2),
                        (e1, '*') => Nonterminal(e1 * *e2),
                        _ => panic!("Unknown operator"),
                    }
                })
            })
        )
    }

    #[test]
    fn test_expression1() {
        let input = vec!['1'];
        let x = expression().parse(& input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal(1)]"
        )
    }

    #[test]
    fn test_expression2() {
        let input = vec!['-', '2'];
        let x = expression().parse(& input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal(-2)]"
        )
    }

    #[test]
    fn test_expression3() {
        let input = vec!['2', '*', '3'];
        let x = expression().parse(& input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal((2 * 3))]"
        )
    }

    #[test]
    fn test_expression4() {
        let input = vec!['-', '2', '+', '3'];
        let x = expression().parse(& input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal((-2 + 3)), Nonterminal(-(2 + 3))]"
        )
    }

    #[test]
    fn test_expression5() {
        let input = vec!['1', '*', '2', '+', '3'];
        let x = expression().parse(& input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal((1 * (2 + 3))), Nonterminal(((1 * 2) + 3))]"
        )
    }

    #[test]
    fn test_expression6a() {
        let input = vec!['1', '*', '2', '+', '-', '3'];
        let x = expression().parse(& input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal((1 * (2 + -3))), Nonterminal(((1 * 2) + -3))]"
        )
    }

    #[test]
    fn test_expression6b() {
        let input = vec!['1', '*', '-', '2', '+', '3'];
        let x = expression().parse(& input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal((1 * (-2 + 3))), Nonterminal((1 * -(2 + 3))), Nonterminal(((1 * -2) + 3))]"
        )
    }
    #[test]
    fn test_zero_characters() {
        let input = vec![];
        let x = express_gen(& expr_item()).parse(&input);
        assert_eq!(
            format!("{:?}", x),
            "[]",
        );
    }

    #[test]
    fn test_one_character() {
        let input = vec!['1'];
        let x = express_gen(& expr_item()).parse(&input);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal(1)]"
        );
    }

    #[test]
    fn test_item() {
        let g = item();
        let x = g.parse(&vec!['a']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('a')));
    }

    #[test]
    fn test_two_characters1() {
        let input = "-1".chars().collect();
        let x = express_gen(& expr_item()).parse(&input);
        assert_eq!("[Nonterminal(-1)]", format!("{:?}", x));
    }

    #[test]
    fn test_two_characters2() {
        let input = "1+".chars().collect();
        let x = express_gen(& expr_item()).parse(&input);
        assert_eq!("[]", format!("{:?}", x));
    }

    #[test]
    fn test_three_characters() {
        let input = "1+3".chars().collect();
        let x = express_gen(& expr_item()).parse(&input);
        assert_eq!(format!("{:?}", x), "[Nonterminal(4)]",)
    }

    #[test]
    fn test_four_characters() {
        let input = "-1+2*4".chars().collect();
        let x: Vec<Grammar<char, i64>> = express_gen(
            & express_gen(& express_gen(& expr_item()))
        )
        .parse(&input)
        .into_iter().filter(|result|
            matches!(result, Grammar::Nonterminal(_))
        )
        .collect();
        assert_eq!(5, x.len(), "{x:?}");
    }

    #[test]
    fn test_item_then() {
        let g = item().then(|c: & char| {
            assert_eq!(*c, 'a');
            Grammar::Nonterminal("success")
        });
        let x = g.parse(&vec!['a']);
        assert_eq!(format!("{:?}", x), "[Nonterminal(\"success\")]",)
    }

    #[test]
    fn test_item_then_item_then1() {
        let g: Grammar<char, &str> = item::<char, char>().then(|c1| {
            let c1 = c1.clone();
            item().then(move |c2: & char| {
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
            .then(|c1: & char| {
                assert_eq!(*c1, 'a');
                item()
            })
            .then(|c2: & char| {
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
            format!("{:?}", digit().or(&letter()).parse(&vec!['a'])),
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
        let g = character('a')
            .or(&character('b'))
            .then(|ab| Grammar::Nonterminal(format!("{ab}")));
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
        let ab =
            character('a').then(|_| character('b').then(|_| Grammar::Nonterminal(format!("ab"))));
        let ac =
            character('a').then(|_| character('c').then(|_| Grammar::Nonterminal(format!("ac"))));
        let g = ab.or(&ac);
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
            "[]"
        );
    }
    #[test]
    fn test_character_plus_parse0() {
        let x = character('a').plus().parse(&vec![]);
        assert_eq!(format!("{:?}", x), "[]");
    }
    #[test]
    fn test_character_star_parse0() {
        let x = character('a').star().parse(&vec![]);
        assert_eq!(format!("{:?}", x), "[Nonterminal([])]");
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
        assert_eq!(format!("{:?}", x), "[Nonterminal(['a'])]");
    }
    #[test]
    fn test_digit_star_parse() {
        assert_eq!(
            format!("{:?}", digit().star().parse(&vec!['1', '2', '3'])),
            "[Nonterminal(['1', '2', '3'])]"
        );
    }
    #[test]
    fn test_digit_or_letter_star_parse() {
        assert_eq!(
            format!(
                "{:?}",
                digit()
                    .or(&letter())
                    .star()
                    .parse(&vec!['a', 'b', 'c', '1', '2', '3'])
            ),
            "[Nonterminal(['a', 'b', 'c', '1', '2', '3'])]"
        );
    }
    #[test]
    fn test_nat_parse() {
        assert_eq!(
            format!("{:?}", nat().parse(&vec!['1', '2', '3',])),
            "[Nonterminal(123)]"
        );
    }
    #[test]
    fn test_integer_parse() {
        assert_eq!(
            format!("{:?}", integer().parse(&vec!['-', '4', '2',])),
            "[Nonterminal(-42)]"
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
            "[Nonterminal(-42)]"
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
