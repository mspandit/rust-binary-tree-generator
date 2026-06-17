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
}

#[derive(Clone)]
pub struct Grammar<T, N>(Rc<dyn Fn(& Stack<T>) -> Vec<(N, Stack<T>)>>)
where T: Debug, N: Debug;

impl<T, N> Grammar<T, N>
where N: Clone + Debug + 'static, T: Clone + Debug + 'static {
    pub fn or(self, alt: Self) -> Self {
        Grammar(Rc::new(move |input: & Stack<T>| {
            let mut results = (self.0)(input);
            println!("{} line {}: Disjoining...", file!(), line!());
            results.extend((alt.0)(input));
            vec![]
        }))
    }

    pub fn then<U>(self, next_f: impl Fn(N) -> Grammar<T, U> + 'static) -> Grammar<T, U>
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

pub fn binary_string1() -> Grammar<char, BinaryString> {
    Grammar(Rc::new(|input|
        if let Some((c, popped)) = input.clone().pop() {
            let retval = vec![(BinaryString(c.to_string()), popped)];
            println!("Returning {:?} on {:?}", retval, input);
            retval
        } else {
            println!("Returning [] on {:?}", input);
            vec![]
        }
    ))
}

pub fn binary_string() -> Grammar<char, BinaryString> {
    binary_string1().or(
        binary_string1().then(move |l|
            binary_string1().then(move |r| {
                let retval = format!("({:?} {:?})", l, r);
                println!("Returning {retval}");
                Grammar::from(BinaryString(retval))
            })
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
