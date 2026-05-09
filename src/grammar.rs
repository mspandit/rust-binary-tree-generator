use std::{fmt::Debug, rc::Rc};

use crate::{Token, context::Context};

pub trait Grammar<T: Token, S> {
     // token -> terminal labels
    fn apply(&self, token: & T) -> Vec<S>;
    fn apply_contextual(&self, context: Context<S>, new: S) -> Vec<Context<S>>;
    fn start(&self, symbol: S) -> bool;
}

pub trait Symbol {
    fn start(&self) -> bool;
}

pub trait Rule<T, S> {
    fn terminal(token: & T) -> Vec<ContextElement<S>>;
    fn nonterminal(left: ContextElement<S>) -> Vec<ContextElement<S>>;
}

#[derive(Clone, Default)]
pub struct BinaryString(String);

impl Symbol for BinaryString {
    fn start(& self) -> bool {
        true
    }
}

impl Debug for BinaryString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub enum ContextElement<T> {
    Complete(T),
    Partial(Rc<dyn Fn(ContextElement<T>) -> Vec<ContextElement<T>>>),
}

// TODO: Instead of passing an instance of the Grammar enum,
// parameterize functions that use the Grammar.
impl<T: Default> Default for ContextElement<T> {
    fn default() -> Self {
        ContextElement::Complete(T::default())
    }
}

impl<T: Debug> Debug for ContextElement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextElement::Complete(value) => write!(f, "{:?}", value),
            ContextElement::Partial(_) => write!(f, "Partial()")
        }
    }
}

impl Rule<char, BinaryString> for ContextElement<BinaryString> {
    // Apply terminal rules to a token, returning the output type
    fn terminal(token: & char) -> Vec<ContextElement<BinaryString>> {
        vec![
            ContextElement::Complete(BinaryString(format!("{}", token)))
        ]
    }

    // Apply nonterminal rules to a complete symbol, returning partial functions
    fn nonterminal(left: ContextElement<BinaryString>) -> Vec<ContextElement<BinaryString>> {
        vec![
            ContextElement::Partial(Rc::new(move |right| vec![
                ContextElement::Complete(BinaryString(format!("({:?} {:?})", left, right)))
            ]))
        ]
    }
}

impl<T: Token, S: Symbol + Clone + 'static> Grammar<T, ContextElement<S>>
for ContextElement<S> where ContextElement<S>: Rule<T, S> {
    fn apply(&self, token: & T) -> Vec<ContextElement<S>> {
        let terminal_results = Self::terminal(token);
        let nonterminal_results = terminal_results.iter().flat_map(|t| Self::nonterminal(t.clone())).collect();
        [terminal_results, nonterminal_results].concat()
    }

    fn apply_contextual(&self, context: Context<ContextElement<S>>, new: ContextElement<S>) -> Vec<Context<ContextElement<S>>> {
        // Apply rule to symbol in context
        match (context.clone().0, new.clone()) {
            (None, _) => {
                // Push new symbol onto empty context and return
                vec![context.clone().push(new.clone())]
            },
            (Some(ref f), _) => {
                let (popped, rest) = f();
                match (popped, new.clone()) {
                    (ContextElement::Complete(_), _) => {
                        // Cannot continue on complete context
                        Vec::default()
                    },
                    (ContextElement::Partial(_f1), ContextElement::Partial(_f2)) => {
                        // Push partial on context awaiting a complete symbol
                        vec![context.clone().push(new)]
                    },
                    (ContextElement::Partial(f), _) => {
                        // Context awaited a complete symbol. Apply it
                        let new_symbols = f(new);
                        // Recurse on the rest of the context with the new symbol, accumulating results
                        let contextual_results = new_symbols.iter().flat_map(|new_symbol|
                            self.apply_contextual(
                                rest.clone(),
                                new_symbol.clone()
                            )
                        ).collect::<Vec<_>>();
                        // Recurse on partials started by the new symbol
                        let recursive_results = new_symbols.iter().flat_map(|new_symbol|
                            Self::nonterminal(new_symbol.clone()).iter()
                            .flat_map(|new_nonterminal|
                                self.apply_contextual(
                                    rest.clone(),
                                    new_nonterminal.clone()
                                )
                            ).collect::<Vec<_>>()
                        ).collect::<Vec<_>>();
                        [contextual_results, recursive_results].concat()
                    },
                }
            }
        }
    }

    fn start(&self, symbol: ContextElement<S>) -> bool {
        match symbol {
            ContextElement::Complete(symbol) => symbol.start(),
            ContextElement::Partial(_) => false
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    UnOp(String),
    E(String),
    BinOp(String),
    EBO(String),
}

impl Symbol for Expression {
    fn start(&self) -> bool {
        match self {
            Expression::E(_) => true,
            _ => false,
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Expression::E(String::default())
    }
}

impl Rule<char, Expression> for ContextElement<Expression> {
    fn terminal(token: & char) -> Vec<ContextElement<Expression>> {
        match token {
            '1' | '2' | '3' | '4' => vec![ContextElement::Complete(Expression::E(format!("{}", token)))],
            '-' => vec![ContextElement::Complete(Expression::UnOp("-".to_string())), ContextElement::Complete(Expression::BinOp("-".to_string()))],
            '+' => vec![ContextElement::Complete(Expression::BinOp("+".to_string()))],
            '*' => vec![ContextElement::Complete(Expression::BinOp("*".to_string()))],
            _ => Vec::default() // No terminal rules
        }
    }

    fn nonterminal(left: ContextElement<Expression>) -> Vec<ContextElement<Expression>> {
        match left.clone() {
            ContextElement::Complete(Expression::UnOp(_)) => {
                vec![ContextElement::Partial(Rc::new(move |right| {
                    println!("Applying nonterminal rule 1 to {:?} and {:?}", left, right);
                    match right.clone() {
                        ContextElement::Complete(Expression::E(_)) => vec![
                            ContextElement::Complete(Expression::E(format!("{:?} {:?}", left, right.clone())))
                        ],
                        _ => vec![]
                    }}))]
            },
            ContextElement::Complete(Expression::E(_)) => {
                vec![ContextElement::Partial(Rc::new(move |right| {
                    println!("Applying nonterminal rule 2 to {:?} and {:?}", left, right);
                    match right.clone() {
                    ContextElement::Complete(Expression::BinOp(_)) => vec![
                        ContextElement::Complete(Expression::EBO(format!("{:?} {:?}", left, right)))
                    ],
                    _ => vec![]
                }}))]
            },
            ContextElement::Complete(Expression::BinOp(_)) => {
                vec![ContextElement::Partial(Rc::new(move |right| {
                    println!("Applying nonterminal rule 3 to {:?} and {:?}", left, right);
                    match right.clone() {
                    ContextElement::Complete(Expression::E(_)) => vec![
                        ContextElement::Complete(Expression::EBO(format!("{:?} {:?}", left, right.clone())))
                    ],
                    _ => vec![]
                }}))]
            },
            ContextElement::Complete(Expression::EBO(_)) => {
                vec![ContextElement::Partial(Rc::new(move |right| {
                    println!("Applying nonterminal rule 4 to {:?} and {:?}", left, right);
                    match right.clone() {
                    ContextElement::Complete(Expression::E(_)) => vec![
                        ContextElement::Complete(Expression::E(format!("{:?} {:?}", left, right.clone())))
                    ],
                    _ => vec![]
            }}))]
            },
            _ => {
                Vec::default()
            }
        }
    }
}

// #[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
// enum _UnOp {
//     #[default]
//     Minus,
// }

// impl Grammar<char, _UnOp> for _UnOp {
//     fn apply(&self, token: & char) -> Vec<_UnOp> {
//         match token {
//             '-' => vec![_UnOp::Minus],
//             _ => Vec::default()
//         }
//     }
// }

// #[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
// enum _BinOp {
//     #[default]
//     Plus,
//     Minus,
//     Times,
// }

// impl Grammar<char, _BinOp> for _BinOp {
//     fn apply(&self, token: & char) -> Vec<_BinOp> {
//         match token {
//             '+' => vec![_BinOp::Plus],
//             '-' => vec![_BinOp::Minus],
//             '*' => vec![_BinOp::Times],
//             _ => Vec::default()
//         }
//     }
// }
// #[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
// pub struct _EBO(Box<_E>, _BinOp);

// impl<E: Token> Grammar<E, _EBO> for _EBO {
//     fn apply(&self, _token: & E) -> Vec<_EBO> {
//         Vec::default() // No terminal rules
//     }
// }
// #[derive(Default, Clone, Eq, PartialEq, Hash, Debug)]
// enum _E {
//     #[default]
//     Zero,
//     One,
//     Two,
//     Three,
//     Four,
//     UnOp(_UnOp, Box<_E>),
//     BinOp(Box<_E>, _BinOp, Box<_E>),
// }

// impl Grammar<char, _E> for _E {
//     fn apply(&self, token: & char) -> Vec<_E> {
//         match token {
//             '0' => vec![_E::Zero],
//             '1' => vec![_E::One],
//             '2' => vec![_E::Two],
//             '3' => vec![_E::Three],
//             '4' => vec![_E::Four],
//             _ => Vec::default()
//         }
//     }
// }

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
