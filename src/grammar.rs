use std::{fmt::Debug, rc::Rc};

#[derive(Clone, Debug)]
pub struct Stack<T>(pub Vec<T>)
where T: Debug;

impl<T> Stack<T>
where T: Clone + Debug {
    pub fn pop(mut self: Self) -> Option<(T, Self)> {
        if let Some(c) = self.0.pop() {
            Some((c, Stack(self.0)))
        } else {
            None
        }
    }

    pub fn is_empty(self: &Self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for Stack<char> {
    fn from(value: String) -> Self {
        Stack(value.chars().rev().collect())
    }
}

impl From<&str> for Stack<char> {
    fn from(value: &str) -> Self {
        Stack(value.to_string().chars().rev().collect())
    }
}

impl From<Vec<&str>> for Stack<String> {
    fn from(value: Vec<&str>) -> Self {
        Stack(value.iter().map(|s|
            s.to_string()).collect::<Vec<String>>()
        )
    }
}

#[derive(Clone)]
pub struct Grammar<T, N>(Rc<dyn Fn(& Stack<T>) -> Vec<(N, Stack<T>)>>)
where T: Debug, N: Debug;

impl<T, N> Grammar<T, N>
where N: Clone + Debug + 'static, T: Clone + Debug + 'static {
    pub fn null() -> Self {
        Grammar(Rc::new(|_| vec![]))
    }

    pub fn any_number(self: Self) -> Grammar<T, Vec<N>> {
        self.clone()
        .one_or_more()
        .or(Grammar::from(vec![]))
    }

    pub fn one_or_more(self: Self) -> Grammar<T, Vec<N>> {
        self.clone()
        .then(move |nonterminal |
            self.clone()
            .any_number()
            .then(move |nonterminals| {
                let mut result = vec![nonterminal.clone()];
                result.extend(nonterminals);
                Grammar::from(result)
            })
        )
    }

    pub fn or(self, alt: Self) -> Self {
        Grammar(Rc::new(move |input: & Stack<T>| {
            let mut results = (self.0)(input);
            println!("{} line {}: Disjoining...", file!(), line!());
            results.extend((alt.0)(input));
            results
        }))
    }

    pub fn then<U>(self, next_f: impl Fn(N) -> Grammar<T, U> + 'static)
    -> Grammar<T, U>
    where U: Debug {
        Grammar(Rc::new(move |input: & Stack<T>| {
            println!("{} line {}: Sequencing...", file!(), line!());
            (self.0)(input)
            .iter()
            .fold(
                vec![],
                |mut result, (nonterminal, stack)| {
                    let grammar = (next_f)(nonterminal.clone());
                    result.extend((grammar.0)(stack));
                    result
                }
            )
        }))
    }

    pub fn from(a: N) -> Self {
        Grammar(Rc::new(move |input: & Stack<T>| vec![
            (a.clone(), (*input).clone())
        ]))
    }

    pub fn parse(self: & Self, input: & Stack<T>) -> Vec<(N, Stack<T>)> {
        println!("Calling parse on {:?}", input);
        (self.0)(input)
    }
}

impl<T, N> Debug for Grammar<T, N>
where T: Debug, N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt:: Result {
        write!(f, "Grammar")
    }
}

#[derive(Default, Clone)]
pub struct BinaryString(String);

impl Debug for BinaryString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn single() -> Grammar<char, BinaryString> {
    Grammar::<char, BinaryString>(Rc::new(|input|
        if let Some((c, popped)) = input.clone().pop() {
            vec![(BinaryString(c.to_string()), popped)]
        } else {
            vec![]
        }
    ))
}

fn double() -> Grammar<char, BinaryString> {
    single().then(|l| binary_string().then(move |r|
            Grammar::from(BinaryString(format!("({:?} {:?})", l, r)))
        )
    )
}

pub fn binary_string() -> Grammar<char, BinaryString> {
    single().or(double())
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

pub fn sentence() -> Grammar<String, Sentence> {
    // Det <-- "the"
    let det: Grammar<String, Sentence> = Grammar(Rc::new(|input|
        if let Some((str, popped)) = input.clone().pop() {
            if str.as_str() == "the" {
                vec![(Sentence::Det(format!("{str}")), popped)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    ));
    // N <-- "cat" | "mat"
    let n: Grammar<String, Sentence> = Grammar(Rc::new(|input|
        if let Some((str, popped)) = input.clone().pop() {
            if str.as_str() == "cat" || str.as_str() == "mat" {
                vec![(Sentence::N(format!("{str}")), popped)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    ));
    // V <-- "sat"
    let v: Grammar<String, Sentence> = Grammar(Rc::new(|input|
        if let Some((str, popped)) = input.clone().pop() {
            if str.as_str() == "sat" {
                vec![(Sentence::V(format!("{str}")), popped)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    ));
    // P <-- "on"
    let p: Grammar<String, Sentence> = Grammar(Rc::new(|input|
        if let Some((str, popped)) = input.clone().pop() {
            if str.as_str() == "on" {
                vec![(Sentence::P(format!("{str}")), popped)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    ));
    // NP <-- Det N
    let np = det.then(move |_| n.clone());
    let np1 = np.clone();
    let np2 = np.clone();
    // PP <-- P NP
    let pp = p.then(move |_| np.clone());
    // VP <-- V NP | V PP
    let vp = v.clone().then(move |_| np1.clone()).or(
        v.clone().then(move |_| pp.clone())
    );
    // S <-- NP VP
    np2.clone().then(move |_| vp.clone())
}

impl Debug for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sentence::Det(s)
            | Sentence::N(s)
            | Sentence::P(s)
            | Sentence::V(s)
            | Sentence::NP(s)
            | Sentence::PP(s)
            | Sentence::VP(s)
            | Sentence::S(s) => write!(f, "{}", s)
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    E(i32),
    UnOp(char),
    BinOp(char),
    EBO(i32, char),
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::UnOp(s) => write!(f, "{}", s),
            Expression::E(s) => write!(f, "{}", s),
            Expression::BinOp(s) => write!(f, "{}", s),
            Expression::EBO(s, op) => write!(f, "({} {})", s, op)
        }
    }
}

fn empty<T, N>() -> Grammar<T, N>
where T: Debug, N: Debug {
    Grammar(Rc::new(|_| vec![]))
}

pub fn expression() -> Grammar<char, Expression> {
    // E <-- 1 | 2 | 3 | 4
    let num = Grammar(Rc::new(|input|
        if let Some((c, popped)) = input.clone().pop() {
            match c {
                '1' | '2' | '3' | '4' => vec![(Expression::E(c.to_digit(10).unwrap() as i32), popped)],
                _ => vec![],
            }
        } else {
            vec![]
        }
    ));
    // UnOp <-- '-'
    let unop = Grammar(Rc::new(|input|
        if let Some((c, popped)) = input.clone().pop() {
            if '-' == c {
                vec![(Expression::UnOp('-'), popped)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    ));
    // BinOp <-- '+' | '*' | '-'
    let binop = Grammar(Rc::new(|input|
        if let Some((c, popped)) = input.clone().pop() {
            match c {
                '+' | '*' | '-' => vec!((Expression::BinOp(c), popped)),
                _ => vec![]
            }
        } else {
            vec![]
        }
    ));
    // EBO <-- E BinOp
    let ebo = empty().then(move |_: char| {
        let binop1 = binop.clone();
        expression().then(move |_| {
            binop1.clone()
        })
    });

    num.or(
        // E <-- UnOp E
        unop.then(|_| expression())
    ).or(
        // E <-- EBO E
        ebo.then(|_| expression())
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let g = Grammar::<char, String>(
            Rc::new(
                |cs| if let Some((c, popped)) = cs.clone().pop() {
                    vec![(c.to_string(), popped)]
                } else {
                    vec![]
                }
            )
        );
        let x = g.parse(&Stack(vec!['a']));
        assert_eq!(1, x.len());
    }

    #[test]
    fn test2() {
        let g = Grammar::<char, String>(
            Rc::new(|_| vec![])
        );
        let x = g.parse(&Stack(vec!['a']));
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_or() {
        let a = Grammar::<char, String>(
            Rc::new(|cs| if let Some((c, popped)) = cs.clone().pop() {
                if 'a' == c {
                    vec![(c.to_string(), popped)]
                } else {
                    println!("{} line {}: returning empty", file!(), line!());
                    vec![]
                }
            } else {
                vec![]
            })
        );
        let b = Grammar::<char, String>(
            Rc::new(|cs| if let Some((c, popped)) = cs.clone().pop() {
                if 'b' == c {
                    vec![(c.to_string(), popped)]
                } else {
                    println!("{} line {}: returning empty", file!(), line!());
                    vec![]
                }
            } else {
                vec![]
            })
        );
        let x = a.clone().or(b.clone()).parse(&"c".into());
        assert_eq!(0, x.len());
        let x = a.clone().or(b.clone()).parse(&"a".into());
        assert_eq!("a".to_string(), x[0].0);
        let x = a.clone().or(b.clone()).parse(&"b".into());
        assert_eq!("b".to_string(), x[0].0);
    }

    #[test]
    fn test_then() {
        let a = Grammar::<char, String>(
            Rc::new(|cs| if let Some((c, popped)) = cs.clone().pop() {
                if 'a' == c {
                    vec![(c.to_string(), popped)]
                } else {
                    println!("{} line {}: returning empty", file!(), line!());
                    vec![]
                }
            } else {
                vec![]
            })
        );
        let b = Grammar::<char, String>(
            Rc::new(|cs| if let Some((c, popped)) = cs.clone().pop() {
                if 'b' == c {
                    vec![(c.to_string(), popped)]
                } else {
                    println!("{} line {}: returning empty", file!(), line!());
                    vec![]
                }
            } else {
                vec![]
            })
        );
        let a1 = a.clone();
        let b1 = b.clone();
        let x = a1.then(move |_| b1.clone()).parse(&"c".into());
        assert_eq!(0, x.len());
        let a2 = a.clone();
        let b2 = b.clone();
        let x = a2.then(move |_| b2.clone()).parse(&"ba".into());
        assert_eq!(0, x.len());
        let x = a.clone().then(move |l|
            b.clone().then(move |r|
                Grammar::from(format!("({l} {r})"))
            )
        ).parse(&"ab".into());
        assert_eq!("(a b)".to_string(), x[0].0);
    }

    fn item() -> Grammar<char, char> {
        Grammar(Rc::new(|input| if let Some((c, popped)) = input.clone().pop() {
                vec![(c, popped)]
            } else {
                vec![]
            }
        ))
    }

    fn sat(pred: impl Fn(char) -> bool + 'static) -> Grammar<char, char> {
        item().then(move |x| {
            if pred(x) {
                Grammar::from(x)
            } else {
                Grammar::null()
            }
        })
    }

    fn character(c: char) -> Grammar<char, char> {
        sat(move |x| x == c)
    }

    fn space() -> Grammar<char, ()> {
        sat(|c| c.is_whitespace())
        .any_number()
        .then(|_whitespace| Grammar::from(()))
    }

    fn token<N: Clone + Debug + 'static>(g: Grammar<char, N>) -> Grammar<char, N> {
        space().then(move |_space|
            g.clone().then(move |tok| {
                space().then(move |_space|
                    Grammar::from(tok.clone())
                )
            })
        )
    }

    fn digit() -> Grammar<char, char> {
        sat(|c| c.is_ascii_digit())
    }

    fn nat() -> Grammar<char, i64> {
        digit()
        .any_number()
        .then(|digits| {
            let n = digits.into_iter()
            .fold(
                0,
                |acc, d|
                    acc * 10 + d.to_digit(10).unwrap() as i64
            );
            Grammar::from(n)
        })
    }

    fn int() -> Grammar<char, i64> {
        character('-').then(|_minus|
            nat().then(|n|
                Grammar::from(-n)
            )
        ).or(nat())
    }

    fn integer() -> Grammar<char, i64> {
        token(int())
    }

    fn factor() -> Grammar<char, i64> {
        character('(').then(move |_open|
            expr().then(|x|
                character(')').then(move |_close|
                    Grammar::from(x)
                )
            )
        ).or(integer())
    }

    fn term() -> Grammar<char, i64> {
        factor().then(|x|
            character('*').then(move |_star|
                term().then(move |y|
                    Grammar::from(x * y)
                )
            )
        ).or(factor())
    }

    fn expr() -> Grammar<char, i64> {
        term().then(move |x|
            character('+').then(move |_plus|
                expr().then(move |y|
                    Grammar::from(x + y)
                )
            )
        ).or(term())
    }

    #[test]
    fn test_digit_parse() {
        assert_eq!(
            format!("{:?}", digit().parse(&"123".into())),
            "[('1', Stack(['3', '2']))]"
        );
    }

    #[test]
    fn test_digit_parse_non_digit() {
        assert_eq!(
            format!("{:?}", digit().parse(&"abc".into())),
            "[]"
        );
    }

    #[test]
    fn test_character_parse() {
        assert_eq!(
            format!("{:?}", character('a').parse(&"abc".into())),
            "[('a', Stack(['c', 'b']))]"
        );
    }

    #[test]
    fn test_character_parse_non_char() {
        assert_eq!(
            format!("{:?}", character('a').parse(&"123".into())),
            "[]"
        );
    }

    #[test]
    fn test_digit_star_parse() {
        let x: Vec<_> = digit()
        .any_number()
        .parse(&"123".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(['1', '2', '3'], Stack([]))]"
        );
    }

    #[test]
    fn test_nat_parse() {
        let x: Vec<_> = nat()
        .parse(&"123".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(123, Stack([]))]"
        );
    }

    #[test]
    fn test_integer_parse() {
        let x: Vec<_> = integer()
        .parse(&"-42".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(-42, Stack([]))]"
        )
    }

    #[test]
    fn test_factor_parse() {
        let x: Vec<_> = factor().parse(&"42".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(42, Stack([]))]"
        );
    }

    #[test]
    fn test_term_parse() {
        let x: Vec<_> = term().parse(&"3*4".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(12, Stack([]))]"
        );
    }

    #[test]
    fn test_expr_parse() {
        let x: Vec<_> = expr().parse(&"2+3*4".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(14, Stack([]))]"
        );
    }

    #[test]
    fn test_expr_parse_with_parentheses1() {
        let x: Vec<_> = expr().parse(&"(2+3)*4".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(20, Stack([]))]"
        );
    }

    #[test]
    fn test_expr_parse_with_parentheses2() {
        let x: Vec<_> = expr().parse(&"(2+(7*10)+8)*20".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(1600, Stack([]))]"
        );
    }

    #[test]
    fn test_expr_parse_fail1() {
        let x: Vec<_> = expr().parse(&"2+3*".into())
        .into_iter()
        .filter(|(_, stack)| stack.0.len() == 1)
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[(5, Stack(['*']))]"
        );
    }

    #[test]
    fn test_expr_parse_fail2() {
        let x: Vec<_> = expr().parse(&"(2+3".into())
        .into_iter()
        .filter(|(_, stack)| stack.is_empty())
        .collect();
        assert_eq!(
            format!("{:?}", x),
            "[]"
        );
    }
}