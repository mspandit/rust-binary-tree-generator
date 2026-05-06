use std::{fmt::Debug, hash::Hash};

use crate::Token;

pub trait Grammar<T: Token, S> {
     // token -> terminal labels
    fn apply(&self, token: & T) -> Vec<S>;
    // (left_label, right_label) -> nonterminal labels
    fn apply_partial(&self, lr: (S, S)) -> Vec<S>;
}

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Binary(String);
impl Grammar<char, Binary> for Binary {
    fn apply(&self, token: & char) -> Vec<Binary> { vec![Binary(format!("{}", token))] }

    fn apply_partial(&self, lr: (Binary, Binary)) -> Vec<Binary> {
        vec![Binary(format!("({} {})", lr.0 .0, lr.1 .0))]
    }
}

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
enum _UnOp {
    #[default]
    Minus,
}

impl Grammar<char, _UnOp> for _UnOp {
    fn apply(&self, token: & char) -> Vec<_UnOp> {
        match token {
            '-' => vec![_UnOp::Minus],
            _ => Vec::default()
        }
    }

    fn apply_partial(&self, _lr: (_UnOp, _UnOp)) -> Vec<_UnOp> {
        Vec::default() // No nonterminal rules
    }
}

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
enum _BinOp {
    #[default]
    Plus,
    Minus,
    Times,
}

impl Grammar<char, _BinOp> for _BinOp {
    fn apply(&self, token: & char) -> Vec<_BinOp> {
        match token {
            '+' => vec![_BinOp::Plus],
            '-' => vec![_BinOp::Minus],
            '*' => vec![_BinOp::Times],
            _ => Vec::default()
        }
    }

    fn apply_partial(&self, _lr: (_BinOp, _BinOp)) -> Vec<_BinOp> {
        Vec::default() // No nonterminal rules
    }
}
#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
pub struct _EBO(Box<_E>, _BinOp);

impl<E: Token> Grammar<E, _EBO> for _EBO {
    fn apply(&self, _token: & E) -> Vec<_EBO> {
        Vec::default() // No terminal rules
    }

    fn apply_partial(&self, _lr: (_EBO, _EBO)) -> Vec<_EBO> {
        Vec::default() // No nonterminal rules
    }
}
#[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
enum _E {
    #[default]
    Zero,
    One,
    Two,
    Three,
    Four,
    UnOp(_UnOp, Box<_E>),
    BinOp(Box<_E>, _BinOp, Box<_E>),
}

impl Grammar<char, _E> for _E {
    fn apply(&self, token: & char) -> Vec<_E> {
        match token {
            '0' => vec![_E::Zero],
            '1' => vec![_E::One],
            '2' => vec![_E::Two],
            '3' => vec![_E::Three],
            '4' => vec![_E::Four],
            _ => Vec::default()
        }
    }
    fn apply_partial(&self, _lr: (_E, _E)) -> Vec<_E> {
        todo!()
    }
}

// https://people.cs.nott.ac.uk/pszgmh/pearl.pdf p. 1
type _Parser<Input, Output> = dyn Fn(Input) -> Vec<(Output, Input)>;

// https://people.cs.nott.ac.uk/pszgmh/pearl.pdf p. 2
fn _pure<Input, Output: Default>() -> Box<_Parser<Input, Output>> {
    Box::new(|input| vec![(Output::default(), input)])
}

fn _bind<Input: 'static, OutputA: 'static, OutputB: 'static>(p: Box<_Parser<Input, OutputA>>, f: impl Fn(OutputA) -> Box<_Parser<Input, OutputB>> + 'static) -> Box<_Parser<Input, OutputB>> {
    Box::new(move |input| {
        let mut results = Vec::new();
        for (a, input) in p(input) {
            results.extend(f(a)(input));
        }
        results
    })
}

// fn main() {
//     let item: Box<Parser<String, char>> = Box::new(|input| {
//         if input.is_empty() {
//             Vec::default()
//         } else {
//             vec![(input.chars().next().unwrap(), input[1..].to_string())]
//         }
//     });
// }
