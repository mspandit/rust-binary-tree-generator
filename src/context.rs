use std::{fmt::Debug, rc::Rc};

use crate::{Token, binary_tree::BinaryTree, grammar::Grammar};

#[derive(Clone)]
pub struct Context<S>(pub Option<Rc<dyn Fn() -> (S, Self)>>);

impl<S> Default for Context<S> {
    fn default() -> Self {
        Self(None)
    }
}

impl<S> Debug for Context<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "Empty"),
            Some(_) => write!(f, "Element()")
        }
    }
}

impl<S: Clone + 'static> Context<S> {
    fn push(self: Self, element: S) -> Self {
        Context(
            Some(Rc::new(
                move || (element.clone(), self.clone())
            ))
        )
    }
}

impl<T: Token + Debug + 'static> Context<BinaryTree<T>> {
    fn shift_reduce0(
        self: Self,
        tree: BinaryTree<T>,
        grammar: &Grammar<T, BinaryTree<T>>,
        mut acc: Vec<Self>) -> Vec<Self> {
        self.0.map_or(
            acc.clone(), // Empty context --> return accumulated contexts
            |ref f| {
                let (popped, rest) = f();
                grammar.apply_partial((popped.clone(), tree.clone())).iter().flat_map(|new_nonterminal| {
                    acc.push(
                        rest
                        .clone()
                        .push(new_nonterminal.clone())
                    );
                    rest.clone().shift_reduce0(
                        new_nonterminal.clone(),
                        grammar,
                        acc.clone()
                    )
                })
                .collect()}
        )
    }

    pub fn shift_reduce(self: & Self, token: & T, grammar: &Grammar<T, BinaryTree<T>>)
    -> Vec<Self> {
        grammar.apply(& token).iter().flat_map(|terminal| {
            self.clone().shift_reduce0(
                terminal.clone(),
                grammar,
                vec![self.clone().push(terminal.clone())]
            )
        })
        .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reduce_second_character() {
        let context = Context(Some(
            Rc::new(
                || (
                    BinaryTree::Terminal {
                        label: "UnOp".to_string(),
                        token: '-',
                    },
                    Context(None)
                )
            )
        ));
        let x = context.shift_reduce(&'1', &Grammar::expression());
        println!("{x:?}");
        match x[1] {
            Context(None) => panic!("Expected a non-empty context, got empty"),
            Context(Some(ref f)) => match f() {
                (ref tree, _) => match tree {
                    BinaryTree::Terminal { label: _, token: _ } => panic!("Expected a nonterminal, got {x:?}"),
                    BinaryTree::Nonterminal { label: _, left: _, right: _ } => (),
                }
            }
        }
    }

    #[test]
    fn test_reduce_third_character() {
        let context = Context(Some(
            Rc::new(|| (
                BinaryTree::Terminal{
                    label: "BinOp".to_string(),
                    token: '+'
                },
                Context(Some(
                    Rc::new(
                        || (
                            BinaryTree::Terminal{
                                label: "E".to_string(),
                                token: '1'
                            },
                            Context(None)
                        )
                    )
                ))
            ))
        ));
        let x = context.shift_reduce( &'2', &Grammar::expression());
        assert_eq!(1, x.len(), "{x:?}");
    }
}