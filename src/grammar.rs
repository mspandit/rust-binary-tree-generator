use std::{fmt::Debug, rc::Rc};

type Unary<T, N> = dyn Fn(& T) -> Option<N>;
type Binary<N> = dyn Fn(& N, & N) -> Option<N>;

#[derive(Default, Clone)]
pub struct Grammar<T, N> {
    unary: Vec<Rc<Unary<T, N>>>,
    binary: Vec<Rc<Binary<N>>>,
}

impl<T, N> Grammar<T, N> {
    pub fn new(unary: Vec<Rc<Unary<T, N>>>, binary: Vec<Rc<Binary<N>>>) -> Self {
        Self {
            unary,
            binary,
        }
    }

    // Apply unary rules to a terminal, returning any number of
    // nonterminals
    pub fn apply_unary(&self, token: & T) -> Vec<N> {
        self.unary.iter()
        .flat_map(|rule| rule(token))
        .collect()
    }

    // Apply binary rules to a pair of nonterminals, returning any number of
    // nonterminals
    pub fn apply_binary(self: & Self, left: & N, right: & N) -> Vec<N> {
        self.binary.iter()
        .flat_map(|rule| rule(left, right))
        .collect()
    }
}

impl<T, N> Debug for Grammar<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grammar with {} unary rules and {} binary rules", self.unary.len(), self.binary.len())
    }
}

#[derive(Debug)]
pub struct PartialGrammar<T, N>(pub Grammar<T, N>);

pub type Partial<N> = dyn Fn(& N) -> Vec<N>;

impl<T, N> PartialGrammar<T, N>
where T: Clone + 'static, N: Clone + 'static {
    pub fn apply_unary(&self, token: & T) -> Vec<N> {
        self.0.apply_unary(token)
    }

    pub fn apply_binary(self: & Self, symbols: & Vec<N>)
    -> Vec<Rc<Partial<N>>> {
        symbols.iter()
        .map(|left| {
            let left = left.clone();
            let grammar = self.0.clone();
            Rc::new(
                move |right: &N| grammar.apply_binary(&left, right)
            ) as Rc<Partial<N>>
        })
        .collect()
    }
}

pub trait Start<S> {
    fn start(&self, s: S) -> bool;
}

#[derive(Default, Clone)]
pub struct BinaryString(String);

pub fn binary_string() -> Grammar<char, BinaryString> {
    let unary = vec![
        Rc::new(|token: & char|
            Some(BinaryString(format!("{}", token)))
        ) as Rc<Unary<char, BinaryString>>
    ];
    let binary = vec![
        Rc::new(|left: & BinaryString, right: & BinaryString|
            Some(BinaryString(format!("({:?} {:?})", left, right)))
        ) as Rc<Binary<BinaryString>>
    ];
    Grammar::new(unary, binary)
}

impl Start<BinaryString> for BinaryString {
    fn start(&self, _: BinaryString) -> bool {
        true
    }
}

impl Debug for BinaryString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

pub fn sentence() -> Grammar<String, Sentence> {
    let unary = vec![
        Rc::new(|token: & String|
            match token.as_str() {
                "the" => Some(Sentence::Det("the".to_string())),
                "cat" | "mat" => Some(Sentence::N(format!("{}", token))),
                "sat" => Some(Sentence::V(format!("{}", token))),
                "on" => Some(Sentence::P(format!("{}", token))),
                _ => None
            }
        ) as Rc<Unary<String, Sentence>>
    ];
    let binary = vec![
        Rc::new(|left: & Sentence, right: & Sentence| match (left, right) {
            (Sentence::Det(_), Sentence::N(_)) => Some(
                Sentence::NP(format!("({:?} {:?})", left, right))
            ),
            (Sentence::V(_), Sentence::NP(_)) => Some(
                Sentence::VP(format!("({:?} {:?})", left, right))
            ),
            (Sentence::V(_), Sentence::PP(_)) => Some(
                Sentence::VP(format!("({:?} {:?})", left, right.clone()))
            ),
            (Sentence::P(_), Sentence::NP(_)) => Some(
                Sentence::PP(format!("({:?} {:?})", left, right.clone()))
            ),
            (Sentence::NP(_), Sentence::VP(_)) => Some(
                Sentence::S(format!("({:?} {:?})", left, right.clone()))
            ),
            _ => None
        }) as Rc<Binary<Sentence>>
     ];
    Grammar::new(unary, binary)
}

impl Start<Sentence> for Sentence {
    fn start(&self, s: Sentence) -> bool {
        match s {
            Sentence::S(_) => true,
            _ => false
        }
    }
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
    UnOp(String),
    E(String),
    BinOp(String),
    EBO(String),
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::UnOp(s) | Expression::E(s) | Expression::BinOp(s) | Expression::EBO(s) => write!(f, "{}", s)
        }
    }
}

pub fn expression() -> Grammar<char, Expression> {
    let unary = vec![
        Rc::new(|token: & char| match token {
            '1' | '2' | '3' | '4' => Some(Expression::E(format!("{}", token))),
            '-' => Some(Expression::UnOp("-".to_string())),
            '+' => Some(Expression::BinOp("+".to_string())),
            '*' => Some(Expression::BinOp("*".to_string())),
            _ => None // No terminal rules
        }) as Rc<Unary<char, Expression>>
    ];
    let binary = vec![
        Rc::new(|left: & Expression, right: & Expression| match (left, right) {
            (Expression::UnOp(_), Expression::E(_)) => Some(
                Expression::E(format!("({:?} {:?})", left, right))
            ),
            (Expression::E(_), Expression::BinOp(_)) => Some(
                Expression::EBO(format!("({:?} {:?})", left, right))
            ),
            (Expression::EBO(_), Expression::E(_)) => Some(
                Expression::E(format!("({:?} {:?})", left, right))
            ),
            _ => None
        }) as Rc<Binary<Expression>>
     ];
    Grammar::new(unary, binary)
}

impl Start<Expression> for Expression {
    fn start(&self, s: Expression) -> bool {
        match s {
            Expression::E(_) => true,
            _ => false
        }
    }
}
