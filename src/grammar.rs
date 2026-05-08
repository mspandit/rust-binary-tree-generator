use std::{fmt::Debug, rc::Rc};

use crate::{Token, context::Context};

pub trait Grammar<T: Token, S> {
     // token -> terminal labels
    fn apply(&self, token: & T) -> Vec<S>;
    fn apply_contextual(&self, context: Context<S>, new: S) -> Vec<Context<S>>;
    fn start(&self, symbol: S) -> bool;
}

#[derive(Clone)]
pub enum BinaryString {
    Complete(String),
    Partial(Rc<dyn Fn(String) -> Vec<BinaryString>>),
}

// TODO: Instead of passing an instance of the Grammar enum,
// parameterize functions that use the Grammar.
impl Default for BinaryString {
    fn default() -> Self {
        BinaryString::Complete(String::default())
    }
}

impl Debug for BinaryString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryString::Complete(str) => write!(f, "{}", str),
            BinaryString::Partial(_) => write!(f, "Partial()")
        }
    }
}

impl BinaryString {
    fn terminal_rule(token: & char) -> BinaryString {
        BinaryString::Complete(format!("{}", token))
    }
    fn nonterminal_rule(left: String) -> impl Fn(String) -> Vec<BinaryString> {
        move |right| vec![BinaryString::Complete(format!("({} {})", left, right))]
    }

    // Apply terminal rules to a token, returning the output type
    fn apply_terminal(&self, token: & char) -> Vec<BinaryString> {
        vec![Self::terminal_rule(token)]
    }

    // Apply nonterminal rules to a complete symbol, returning partial functions
    fn apply_nonterminal(&self, binary: BinaryString) -> Vec<BinaryString> {
        match binary {
            BinaryString::Complete(str) => vec![BinaryString::Partial(Rc::new(Self::nonterminal_rule(str)))],
            BinaryString::Partial(_) => Vec::default(), // No nonterminal rules apply to partial symbols
        }
    }
}

impl Grammar<char, BinaryString> for BinaryString {
    fn apply(&self, token: & char) -> Vec<BinaryString> {
        let terminal_results = self.apply_terminal(token);
        let nonterminal_results = terminal_results.iter().flat_map(|t| self.apply_nonterminal(t.clone())).collect();
        [terminal_results, nonterminal_results].concat()
    }

    fn apply_contextual(&self, context: Context<BinaryString>, new: BinaryString) -> Vec<Context<BinaryString>> {
        // Apply rule to symbol in context
        match (context.clone().0, new.clone()) {
            (None, _) => {
                // Push new symbol onto empty context and return
                vec![context.clone().push(new.clone())]
            },
            (Some(ref f), _) => {
                let (popped, rest) = f();
                match (popped, new.clone()) {
                    (BinaryString::Complete(_), _) => {
                        // Cannot continue on complete context
                        Vec::default()
                    },
                    (BinaryString::Partial(f), BinaryString::Complete(right)) => {
                        // Context awaited a complete symbol. Apply it
                        let new_symbols = f(right.clone());
                        // Recurse on the rest of the context with the new symbol, accumulating results
                        let contextual_results = new_symbols.iter().flat_map(|new_symbol|
                            self.apply_contextual(
                                rest.clone(),
                                new_symbol.clone()
                            )
                        ).collect::<Vec<_>>();
                        // Recurse on partials started by the new symbol
                        let recursive_results = new_symbols.iter().flat_map(|new_symbol|
                            self.apply_nonterminal(new_symbol.clone()).iter()
                            .flat_map(|new_nonterminal|
                                self.apply_contextual(
                                    rest.clone(),
                                    new_nonterminal.clone()
                                )
                            ).collect::<Vec<_>>()
                        ).collect::<Vec<_>>();
                        [contextual_results, recursive_results].concat()
                    },
                    (BinaryString::Partial(_f1), BinaryString::Partial(_f2)) => {
                        // Push partial on context awaiting a complete symbol
                        vec![context.clone().push(new)]
                    }
                }
            }
        }
    }

    fn start(&self, symbol: BinaryString) -> bool {
        match symbol {
            BinaryString::Complete(_) => true,
            BinaryString::Partial(_) => false
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Complete(String),
    Partial(Rc<dyn Fn(String) -> Expression>),
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Complete(String::default())
    }
}

impl Expression {
    fn terminal_rule(token: & char) -> Vec<Expression> {
        match token {
            '1' | '2' | '3' | '4' => vec![Expression::Complete(format!("$E({})", token))],
            '-' => vec![Expression::Complete("$UnOp(-)".to_string()), Expression::Complete("$BinOp(-)".to_string())],
            '+' => vec![Expression::Complete("$BinOp(+)".to_string())],
            '*' => vec![Expression::Complete("$BinOp(*)".to_string())],
            _ => Vec::default() // No terminal rules
        }
    }
    // fn nonterminal_rule(left: Expression) -> impl Fn(Expression) -> Expression {
    //     match left {
    //         Expression::Complete(str) if str.starts_with("$UnOp") => {
    //             move |right| Expression::Complete(format!("$E($str {})", right))
    //         },
    //         Expression::Complete(str) if str.starts_with("$BinOp") => {
    //             move |right| Expression::Complete(format!("$E({} {})", str, right))
    //         },
    //         _ =>
    //     move |right| Expression::Complete(format!("({} {})", left, right))
    // }
    // Apply terminal rules to a token, returning the output type
    fn apply_terminal(&self, token: & char) -> Vec<Expression> {
        Self::terminal_rule(token)
    }

    // Apply nonterminal rules to a complete symbol, returning partial functions
    // fn apply_nonterminal(&self, binary: BinaryString) -> Vec<BinaryString> {
    //     match binary {
    //         BinaryString::Complete(str) => vec![BinaryString::Partial(Rc::new(Self::nonterminal_rule(str)))],
    //         BinaryString::Partial(_) => Vec::default(), // No nonterminal rules apply to partial symbols
    //     }
    // }
}

impl Grammar<char, Expression> for Expression {
    fn apply(&self, token: & char) -> Vec<Expression> {
        Expression::terminal_rule(token)
    }

    fn apply_contextual(&self, _context: Context<Expression>, _new: Expression) -> Vec<Context<Expression>> {
        Vec::default() // No nonterminal rules
    }

    fn start(&self, _symbol: Expression) -> bool {
        false // No start symbols
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
