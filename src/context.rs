use std::{fmt::Debug, rc::Rc};

use crate::{Token, grammar::Grammar};

#[derive(Clone)]
pub struct Context<S>(pub Option<Rc<dyn Fn() -> (S, Self)>>);

impl<S> Default for Context<S> {
    fn default() -> Self {
        Self(None)
    }
}

impl<S: Debug> Debug for Context<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "Empty"),
            Some(g) => write!(f, "Element({:?})", g())
        }
    }
}

impl<S: Clone + 'static> Context<S> {
    pub fn push(self: Self, element: S) -> Self {
        Context(
            Some(Rc::new(
                move || (element.clone(), self.clone())
            ))
        )
    }
}

impl<S: Clone + Debug + 'static> Context<S> {
    pub fn shift_reduce<T: Token>(self: & Self, token: & T, grammar: &dyn Grammar<T, S>)
    -> Vec<Self> {
        grammar.apply(token).iter().flat_map(|symbol| {
            grammar.apply_contextual(self.clone(), symbol.clone())
        })
        .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_reduce_second_character() {
    //     let context = Context(Some(
    //         Rc::new(
    //             || (
    //                 BinaryTree::Terminal {
    //                     label: "UnOp".to_string(),
    //                     token: '-',
    //                 },
    //                 Context(None)
    //             )
    //         )
    //     ));
    //     let x = context.shift_reduce(&'1', &Grammar::expression());
    //     println!("{x:?}");
    //     match x[1] {
    //         Context(None) => panic!("Expected a non-empty context, got empty"),
    //         Context(Some(ref f)) => match f() {
    //             (ref tree, _) => match tree {
    //                 BinaryTree::Terminal { label: _, token: _ } => panic!("Expected a nonterminal, got {x:?}"),
    //                 BinaryTree::Nonterminal { label: _, left: _, right: _ } => (),
    //             }
    //         }
    //     }
    // }

    // #[test]
    // fn test_reduce_third_character() {
    //     let context = Context(Some(
    //         Rc::new(|| (
    //             BinaryTree::Terminal{
    //                 label: "BinOp".to_string(),
    //                 token: '+'
    //             },
    //             Context(Some(
    //                 Rc::new(
    //                     || (
    //                         BinaryTree::Terminal{
    //                             label: "E".to_string(),
    //                             token: '1'
    //                         },
    //                         Context(None)
    //                     )
    //                 )
    //             ))
    //         ))
    //     ));
    //     let x = context.shift_reduce( &'2', &Grammar::expression());
    //     assert_eq!(3, x.len(), "{x:?}");
    // }
}