use std::{fmt::Debug, rc::Rc};

#[derive(Clone)]
struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn pop(self: & Self) -> Option<(T, Self)> {
        if let Some(c) = self.0.chars().next() {
            Some((c, Stack((&self.0[1..]).to_string())))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Grammar<T, N>(Rc<dyn Fn(& Stack<T>) -> Vec<(N, Stack<T>)>>);

impl<T, N> Grammar<T, N>
where N: Clone + 'static {
    pub fn or(self, alternative: Self) -> Self {
        Grammar(Rc::new(move |input: & Stack| {
            let mut results = (self.0)(input);
            results.extend((alternative.0)(input));
            vec![]
        }))
    }

    pub fn then<U>(self, next_f: impl Fn(N) -> Grammar<T, U> + 'static) -> Grammar<T, U> {
        Grammar(Rc::new(move |input: & Stack<T>|
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
        ))
    }

    pub fn from(a: N) -> Self {
        Grammar(Rc::new(move |input: & Stack| vec![
            (a.clone(), (*input).clone())
        ]))
    }
}

#[derive(Default, Clone)]
pub struct BinaryString(String);

pub fn binary_string() -> Grammar<char, BinaryString> {
    let s: Grammar<char, BinaryString> = Grammar(Rc::new(|input|
        if let Some((c, popped)) = input.pop() {
            vec![(BinaryString(c.to_string()), popped)]
        } else {
            vec![]
        }
    ));
    s.clone().or(
        s.clone().then(move |l|
            s.clone().then(move |r|
                Grammar::from(BinaryString(format!("({:?} {:?})", l, r)))
            )
        )
    )
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
        if let Some((str, popped)) = input.pop() {
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
        if let Some((str, popped)) = input.pop() {
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
        if let Some((str, popped)) = input.pop() {
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
        if let Some((str, popped)) = input.pop() {
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
    let np = det.then(|_| n);
    // PP <-- P NP
    let pp = p.then(|_| np);
    // VP <-- V NP | V PP
    let vp = v.then(|_| np).or(v.then(|_| pp));
    // S <-- NP VP
    np.then(|_| vp)
}

impl Debug for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sentence::Det(s) | Sentence::N(s) | Sentence::P(s) | Sentence::V(s) | Sentence::NP(s) | Sentence::PP(s) | Sentence::VP(s) | Sentence::S(s) => write!(f, "{}", s)
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

pub fn expression() -> Grammar<char, Expression> {
    // E <-- 1 | 2 | 3 | 4
    let num = Grammar(Rc::new(|input|
        if let Some((c, popped)) = input.pop() {
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
        if let Some((c, popped)) = input.pop() {
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
        if let Some((c, popped)) = input.pop() {
            match c {
                '+' | '*' | '-' => vec!((Expression::BinOp(c), popped)),
                _ => vec![]
            }
        } else {
            vec![]
        }
    ));
    // EBO <-- E BinOp
    let ebo = expression().then(|_| binop);
    num.or(
        // E <-- UnOp E
        unop.then(|_| expression())
    ).or(
        // E <-- EBO E
        ebo.then(|_| expression())
    )
}
