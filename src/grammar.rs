use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub enum ParseResult<T, N> {
    Nonterminal(N),
    Cont(Grammar<T, N>),
}

#[derive(Clone)]
pub enum Grammar<T, N> {
    Shift(Rc<dyn Fn(& T) -> Vec<ParseResult<T, N>>>),
    Reduce(Rc<dyn Fn() -> Vec<ParseResult<T, N>>>),
}

impl<T, N> Debug for Grammar<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grammar::Shift(_) => write!(f, "Grammar::Shift"),
            Grammar::Reduce(_) => write!(f, "Grammar::Reduce"),
        }
    }
}

impl<T, N> Grammar<T, N>
where T: Clone + 'static + Debug, N: Clone + 'static + Debug {
    // Consumes no input and produces no output
    pub fn null() -> Self {
        Grammar::Reduce(Rc::new(|| vec![]))
    }

    pub fn apply(self: & Self, token: & T) -> Vec<ParseResult<T, N>> {
        use Grammar::*;
        match self {
            Shift(g) => g(token),
            Reduce(g) => g(),
        }
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

    // A _choice_ operator. The resulting grammar applies
    // self and other to the input, and returns both results.
    pub fn or(self: Self, other: Self) -> Self {
        use Grammar::*;
        match (self.clone(), other.clone()) {
            (Shift(self_0), Shift(other_0)) => Shift(
                Rc::new(move |input: & T| {
                    let mut results = (self_0)(input);
                    results.extend((other_0)(input));
                    results
                }
            )),
            (Shift(_), Reduce(other_0)) => Reduce(
                Rc::new(move || {
                    let mut results = vec![ParseResult::Cont(self.clone())];
                    results.extend((other_0)());
                    results
                }
            )),
            (Reduce(self_0), Shift(_)) => Reduce(
                Rc::new(move || {
                    let mut results = (self_0)();
                    results.push(ParseResult::Cont(other.clone()));
                    results
                }
            )),
            (Reduce(self_0), Reduce(other_0)) => Reduce(
                Rc::new(move || {
                    let mut results = (self_0)();
                    results.extend((other_0)());
                    results
                }
            )),
        }
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
        match self.clone() {
            Shift(self_0) => Shift(Rc::new(move |input: & T| {
                (self_0)(input)
                .iter()
                .flat_map(
                    |result| {
                        match result {
                            ParseResult::Cont(g) => vec![
                                ParseResult::Cont(g.clone().then(f.clone()))
                            ],
                            ParseResult::Nonterminal(n) => {
                                let g = f(n.clone());
                                match g.clone() {
                                    Shift(_) => vec![ParseResult::Cont(g)],
                                    Reduce(g_0) => g_0(),
                                }
                            },
                        }
                    }
                )
                .collect()
            })),
            Reduce(self_0) => Reduce(Rc::new(move || {
                self_0()
                .iter()
                .flat_map(
                    |result| {
                        match result {
                            ParseResult::Cont(g) => vec![
                                ParseResult::Cont(g.clone().then(f.clone()))
                            ],
                            ParseResult::Nonterminal(n) => {
                                let g = f(n.clone());
                                match g.clone() {
                                    Shift(_) => vec![ParseResult::Cont(g)],
                                    Reduce(g_0) => g_0()
                                }
                            },
                        }
                    }
                )
                .collect()
            })),
        }
    }

    pub fn parse(self: & Self, input_sequence: & Vec<T>) -> Vec<ParseResult<T, N>> {
        input_sequence.iter().fold(
            match self {
                    Grammar::Shift(_) => vec![ParseResult::Cont(self.clone())],
                    Grammar::Reduce(g) => g(),
            },
            |state, token| {
                state.into_iter().flat_map(|context|
                    match context {
                        ParseResult::Nonterminal(_) => {
                            vec![]
                        },
                        ParseResult::Cont(g) => {
                            use Grammar::*;
                            match g {
                                Shift(ref g_0) => {
                                    (g_0)(token)
                                },
                                Reduce(_) => unreachable!("Grammar::Reduce not implemented"),
                            }
                        },
                    }
                )
                .collect()
            }
        )
    }
}

fn recurse<T: 'static, N: 'static>(f: &'static dyn Fn(& Grammar<T, N>, & T) -> Vec<ParseResult<T, N>>, x: & T) -> Vec<ParseResult<T, N>> {
    f(& Grammar::Shift(Rc::new(|y: & T| recurse(f, y))), x)
}

pub fn recursive<T: 'static, N: 'static>(f: &'static dyn Fn(& Grammar<T, N>, & T) -> Vec<ParseResult<T, N>>) -> Grammar<T, N> {
    Grammar::Shift(Rc::new(move |x: &T| recurse(f, x)))
}

// Implements the monadic return operator, returning a grammar
// that returns the given value without consuming any of the
// input.
impl<T, N> From<N> for Grammar<T, N>
where N: Clone + 'static + Debug {
    fn from(value: N) -> Self {
        Grammar::Reduce(Rc::new(move || {
            println!("{} line {}: returning {:?}", file!(), line!(), value);
            vec![ParseResult::Nonterminal(value.clone())]
        }))
    }
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
    let cat = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "cat" {
            vec![ParseResult::Nonterminal(Sentence::N(token.clone()))]
        } else {
            vec![]
        }
    ));
    let mat = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "mat" {
            vec![ParseResult::Nonterminal(Sentence::N(token.clone()))]
        } else {
            vec![]
        }
    ));
    cat.or(mat)
}

pub fn noun_phrase() -> Grammar<String, Sentence> {
    let det = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "the" {
            vec![ParseResult::Nonterminal(Sentence::Det(token.clone()))]
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
    let v = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "sat" {
            vec![ParseResult::Nonterminal(Sentence::V(token.clone()))]
        } else {
            vec![]
        }
    ));
    let p = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "on" {
            vec![ParseResult::Nonterminal(Sentence::P(token.clone()))]
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
    Grammar::Shift(Rc::new(|input: &char| {
        vec![ParseResult::Nonterminal(*input)]
    }))
}

fn sat(p: impl Fn(char) -> bool + 'static + Clone) -> Grammar<char, char> {
    item().then(move |x| {
        if p(x) {
            Grammar::<char, char>::from(x)
        } else {
            Grammar::null()
        }
    })
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
        assert_eq!(
            format!("{:?}", character('a').then(|_| character('a')).then(|_| Grammar::from("success")).parse(& vec!['a', 'a'])),
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
            "[Cont(Grammar::Shift), Cont(Grammar::Shift)]"
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
            "[Cont(Grammar::Shift), Cont(Grammar::Shift)]"
        );
    }
    #[test]
    fn test_character_plus_parse0() {
        let x = character('a').plus().parse(& vec![]);
        assert_eq!(
            format!("{:?}", x),
            "[Cont(Grammar::Shift)]"
        );
    }
    #[test]
    fn test_character_star_parse0() {
        let x = character('a')
        .star()
        .parse(& vec![]);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal([]), Cont(Grammar::Shift)]"
        );
    }
    #[test]
    fn test_character_plus_parse1() {
        let x = character('a').plus().parse(& vec!['a']);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal(['a']), Cont(Grammar::Shift)]"
        );
    }
    #[test]
    fn test_digit_star_parse() {
        assert_eq!(
            format!("{:?}", digit().star().parse(& vec!['1', '2', '3'])),
            "[Nonterminal(['1', '2', '3']), Cont(Grammar::Shift)]"
        );
    }
   #[test]
    fn test_digit_or_letter_star_parse() {
        assert_eq!(
            format!("{:?}", digit().or(letter()).star().parse(& vec!['a', 'b', 'c', '1', '2', '3'])),
            "[Nonterminal(['a', 'b', 'c', '1', '2', '3']), Cont(Grammar::Shift)]"
        );
    }
    #[test]
    fn test_nat_parse() {
        assert_eq!(
            format!("{:?}", nat().parse(& vec!['1', '2', '3', ])),
            "[Nonterminal(123), Cont(Grammar::Shift)]"
        );
    }
   #[test]
    fn test_integer_parse() {
        assert_eq!(
            format!("{:?}", integer().parse(& vec!['-', '4', '2', ])),
            "[Nonterminal(-42), Cont(Grammar::Shift), Cont(Grammar::Shift)]"
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
            "[Nonterminal(-42), Cont(Grammar::Shift), Cont(Grammar::Shift)]"
        );
    }
    #[test]
    fn test_term_parse() {
        let input = "3*4".chars().collect();
        let r: Vec<ParseResult<char, i64>> = term().parse(& input)
            .into_iter().filter(|pr| matches!(pr, ParseResult::Nonterminal(_)))
            .collect();
        assert_eq!(
            format!("{:?}", r),
            "[Nonterminal(12)]"
        );
    }
    #[test]
    fn test_expr_parse() {
        let r: Vec<ParseResult<char, i64>> = expr()
            .parse(& vec!['2', '+', '3', '*', '4', ])
            .into_iter().filter(|pr| matches!(pr, ParseResult::Nonterminal(_)))
            .collect();
        assert_eq!(format!("{:?}", r), "[Nonterminal(14)]");
    }
    #[test]
    fn test_expr_parse_with_parentheses1() {
        let input: Vec<char> = "(2+3)*4".chars().collect();
        let r: Vec<ParseResult<char, i64>> = expr()
        .parse(& input)
        .into_iter()
        .filter(|pr| matches!(pr, ParseResult::Nonterminal(_)))
        .collect();
        assert_eq!(
            format!("{:?}", r),
            "[Nonterminal(20)]"
        );
    }
    #[test]
    fn test_expr_parse_with_parentheses2() {
        let input: Vec<char> = "(2+(7*10)+8)*20".chars().collect();
        let r: Vec<ParseResult<char, i64>> = expr()
        .parse(& input)
        .into_iter()
        .filter(|pr| matches!(pr, ParseResult::Nonterminal(_)))
        .collect();
        assert_eq!(
            format!("{:?}", r),
            "[Nonterminal(1600)]"
        );
    }

    #[test]
    fn test_expr_parse_fail1() {
        let input = "2+3*".chars().collect();
        let r: Vec<ParseResult<char, i64>> = expr()
        .parse(& input)
        .into_iter()
        .filter(|pr| matches!(pr, ParseResult::Nonterminal(_)))
        .collect();
        assert_eq!(
            format!("{:?}", r),
            "[]"
        );
    }

    #[test]
    fn test_expr_parse_fail2() {
        let input = "(2+3".chars().collect();
        let r: Vec<ParseResult<char, i64>> = expr().parse(& input)
        .into_iter()
        .filter(|pr| matches!(pr, ParseResult::Nonterminal(_)))
        .collect();
        assert_eq!(
            format!("{:?}", r),
            "[]"
        );
    }
}