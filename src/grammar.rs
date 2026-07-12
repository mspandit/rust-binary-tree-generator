use std::{fmt::Debug, rc::Rc};

pub enum Grammar<T, N> {
    Nonterminal(N),
    Continuation(Rc<dyn Fn(& T) -> Vec<Grammar<T, N>>>),
}

impl<T, N> Grammar<T, N>
where T: Clone + 'static + Debug, N: Clone + 'static + Debug {
    pub fn apply(self: & Self, token: & T) -> Vec<Grammar<T, N>> {
        use Grammar::*;
        match self {
            Nonterminal(_) => vec![],
            Continuation(own_f) => own_f(token),
        }
    }

    // A _choice_ operator. The resulting grammar applies
    // self and other to the input, and returns both results.
    pub fn or(self: Self, other: Self) -> Self {
        Grammar::Continuation(Rc::new(move |input: & T| {
            let mut results = self.apply(input);
            results.extend(other.apply(input));
            results
        }))
    }

    // A _sequence_ operator. The resulting grammar applies
    // self to an input to give a list of results.
    //
    // If a result is a Nonterminalinal n, then f(n) returns a
    // grammar to be applied to a subsequent input. It
    // is returned as a ParseResult::Nonterminal.
    //
    // If a result is a Nonterminal g, then g must be
    // applied to the subsequent input first. g.then(f) is
    // returned as a ParseResult::Nonterminal.
    pub fn then<F, U>(self: & Self, f: F) -> Grammar<T, U>
    where F: Fn(N) -> Grammar<T, U> + Clone + 'static,
        U: Clone {
        use Grammar::*;
        let self_clone = self.clone();
        Continuation(Rc::new(move |token: & T| {
            self_clone.apply(token).iter().flat_map(
                |own_f_res| {
                    match own_f_res {
                        Continuation(_) => vec![own_f_res.then(f.clone())],
                        Nonterminal(n) => vec![f(n.clone())],
                    }
                }
            )
            .collect()
        }))
    }

    // Kleene star operator. The resulting grammar applies
    // self to the input zero or more times, returning a
    // vector of the results.
    pub fn star(self: Self) -> Grammar<T, Vec<N>> {
        Grammar::from(vec![])
        .or(self.clone().plus())
    }

    // Kleene plus operator. The resulting grammar applies
    // self to the input one or more times, returning a
    // vector of the results.
    pub fn plus(self: Self) -> Grammar<T, Vec<N>> {
        self.clone()
        .then(move |a| self.clone()
            .star()
            .then(move |a_s| {
                let mut result = vec![a.clone()];
                result.extend(a_s);
                Grammar::from(result)
            })
        )
    }

    pub fn parse(self: & Self, input_sequence: & Vec<T>) -> Vec<Grammar<T, N>> {
        use Grammar::*;
        match self {
            Nonterminal(_) => vec![self.clone()],
            Continuation(_) => input_sequence.iter().fold(
                vec![self.clone()],
                |state, token| {
                    state.into_iter()
                    .flat_map(|context| match context {
                        Nonterminal(_) => vec![],
                        Continuation(g) => g(token)
                    })
                    .collect()
                }
            )
        }
    }
}

impl<T, N> Clone for Grammar<T, N>
where N: Clone {
    fn clone(&self) -> Self {
        match self {
            Self::Nonterminal(n) => Self::Nonterminal(n.clone()),
            Self::Continuation(f) => Self::Continuation(f.clone()),
        }
    }
}

impl<T, N> Debug for Grammar<T, N>
where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grammar::Continuation(_) => write!(f, "Continuation"),
            Grammar::Nonterminal(n) => write!(f, "Nonterminal({n:?})"),
        }
    }
}

// Implements the monadic return operator, returning a grammar
// that returns the given value without consuming any of the
// input.
impl<T, N> From<N> for Grammar<T, N>
where N: Clone + 'static + Debug {
    fn from(value: N) -> Self {
        Grammar::Nonterminal(value.clone())
    }
}

fn recurse<T: 'static, N: 'static>(f: &'static dyn Fn(& Grammar<T, N>, & T) -> Vec<Grammar<T, N>>, x: & T) -> Vec<Grammar<T, N>> {
    f(& Grammar::Continuation(Rc::new(|y: & T| recurse(f, y))), x)
}

pub fn recursive<T: 'static, N: 'static>(f: &'static dyn Fn(& Grammar<T, N>, & T) -> Vec<Grammar<T, N>>) -> Grammar<T, N> {
    Grammar::Continuation(Rc::new(move |x: &T| recurse(f, x)))
}

#[derive(Clone)]
pub enum Sentence {
    Det(String),
    N(String),
    P(String),
    V(String),
    NP(String),
    PP(String),
    VP(String),
    S(String),
}

pub fn noun() -> Grammar<String, Sentence> {
    let cat = Grammar::Continuation(Rc::new(|token: & String|
        if token.as_str() == "cat" {
            vec![Grammar::Nonterminal(Sentence::N(token.clone()))]
        } else {
            vec![]
        }
    ));
    let mat = Grammar::Continuation(Rc::new(|token: & String|
        if token.as_str() == "mat" {
            vec![Grammar::Nonterminal(Sentence::N(token.clone()))]
        } else {
            vec![]
        }
    ));
    cat.or(mat)
}

pub fn noun_phrase() -> Grammar<String, Sentence> {
    let det = Grammar::Continuation(Rc::new(|token: & String|
        if token.as_str() == "the" {
            vec![Grammar::Nonterminal(Sentence::Det(token.clone()))]
        } else {
            vec![]
        }
    ));
    det.then(
        move |d| {
            let d = d.clone();
            noun().then(move |n_sym| {
                Grammar::from(Sentence::NP(format!("({:?} {:?})", d.clone(), n_sym.clone())))
            })
        }
    )
}

pub fn sentence() -> Grammar<String, Sentence> {
    let v = Grammar::Continuation(Rc::new(|token: & String|
        if token.as_str() == "sat" {
            vec![Grammar::Nonterminal(Sentence::V(token.clone()))]
        } else {
            vec![]
        }
    ));
    let p = Grammar::Continuation(Rc::new(|token: & String|
        if token.as_str() == "on" {
            vec![Grammar::Nonterminal(Sentence::P(token.clone()))]
        } else {
            vec![]
        }
    ));
    let np = noun_phrase();

    let pp = p.clone().then({
        let np = np.clone();
        move |p_sym| {
            let p_sym = p_sym.clone();
            np.clone().then(move |np_sym| {
                Grammar::from(Sentence::PP(format!("({:?} {:?})", p_sym.clone(), np_sym.clone())))
            })
        }
    });

    let vp = v.clone()
    .then({
        let np = np.clone();
        move |v_sym| {
            let v_sym = v_sym.clone();
            np.clone().then(move |np_sym| {
                Grammar::from(Sentence::VP(format!("({:?} {:?})", v_sym.clone(), np_sym.clone())))
            })
        }
    })
    .or(v.then({
        let pp = pp.clone();
        move |v_sym| {
            let v_sym = v_sym.clone();
            pp.clone().then(move |pp_sym| {
                Grammar::from(Sentence::VP(format!("({:?} {:?})", v_sym.clone(), pp_sym.clone())))
            })
        }
    }));

    np.then(move |np_sym| {
        let np_sym = np_sym.clone();
        vp.clone().then(move |vp_sym| {
            Grammar::from(Sentence::S(format!("({:?} {:?})", np_sym.clone(), vp_sym.clone())))
        })
    })
}

impl Debug for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sentence::Det(s) | Sentence::N(s) | Sentence::P(s) | Sentence::V(s) | Sentence::NP(s) | Sentence::PP(s) | Sentence::VP(s) | Sentence::S(s) => write!(f, "{}", s)
        }
    }
}

fn item() -> Grammar<char, char> {
    Grammar::Continuation(Rc::new(|input: &char| {
        vec![Grammar::Nonterminal(*input)]
    }))
}

fn sat(p: impl Fn(char) -> bool + 'static + Clone) -> Grammar<char, char> {
    Grammar::Continuation(Rc::new(move |input: &char| {
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

pub fn binary_string() -> Grammar<char, String> {
    recursive(&|binary_string: &Grammar<char, String>, x: &char| {
        let binary_string1 = binary_string.clone();
        binary_string1.clone().then(move |l|{
            let binary_string2 = binary_string1.clone();
            binary_string2.then(move |r|
                format!("({l} {r})").to_string().into()
            )
        })
        .or(x.to_string().into()).apply(x)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_grammar1() {
        let g: Grammar<char, char> = Grammar::Nonterminal('a');
        let x = g.parse(&vec![]);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('a')));
    }

    #[test]
    fn test_simple_grammar2() {
        let g: Grammar<char, char> = Grammar::Nonterminal('b');
        let x = g.parse(&vec![]);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('b')));
    }

    #[test]
    fn test_simple_grammar3() {
        let g: Grammar<char, char> = Grammar::Continuation(Rc::new(|t| if 'c' == *t {
            vec![Grammar::Nonterminal('c')]
        } else {
            vec![]
        }));
        let x = g.parse(&vec!['c']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('c')));
    }

    #[test]
    fn test_item() {
        let g = item();
        let x = g.parse(&vec!['a']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('a')));
    }

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
            "[Continuation, Continuation]"
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
            "[Continuation, Continuation]"
        );
    }
    #[test]
    fn test_character_plus_parse0() {
        let x = character('a')
        .plus()
        .parse(& vec![]);
        assert_eq!(
            format!("{:?}", x),
            "[Continuation]"
        );
    }
    #[test]
    fn test_from() {
        let x: Vec<Grammar<char, Vec<char>>> = Grammar::from(vec![])
        .parse(& vec![]);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal([])]"
        )
    }
    #[test]
    fn test_character_star_parse0() {
        let x = character('a')
        .star()
        .parse(& vec![]);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal([]), Continuation]"
        );
    }
    #[test]
    fn test_character_plus_parse1() {
        let x = character('a').plus().parse(& vec!['a']);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal(['a']), Continuation]"
        );
    }
    #[test]
    fn test_digit_star_parse() {
        assert_eq!(
            format!("{:?}", digit().star().parse(& vec!['1', '2', '3'])),
            "[Nonterminal(['1', '2', '3']), Cont(Grammar::Continuation)]"
        );
    }
   #[test]
    fn test_digit_or_letter_star_parse() {
        assert_eq!(
            format!("{:?}", digit().or(letter()).star().parse(& vec!['a', 'b', 'c', '1', '2', '3'])),
            "[Nonterminal(['a', 'b', 'c', '1', '2', '3']), Cont(Grammar::Continuation)]"
        );
    }
    #[test]
    fn test_nat_parse() {
        assert_eq!(
            format!("{:?}", nat().parse(& vec!['1', '2', '3', ])),
            "[Nonterminal(123), Cont(Grammar::Continuation)]"
        );
    }
   #[test]
    fn test_integer_parse() {
        assert_eq!(
            format!("{:?}", integer().parse(& vec!['-', '4', '2', ])),
            "[Nonterminal(-42), Cont(Grammar::Continuation), Cont(Grammar::Continuation)]"
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
            "[Nonterminal(-42), Cont(Grammar::Continuation), Cont(Grammar::Continuation)]"
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