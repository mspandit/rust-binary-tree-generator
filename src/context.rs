use std::{fmt::{Debug, Display}, rc::Rc};

use crate::{Token, binary_tree::BinaryTree, grammar::Grammar};

#[derive(Clone)]
pub struct Context<S: Debug + Clone>(pub Option<Rc<dyn Fn() -> (S, Self)>>);

impl<S: Clone + Debug> Default for Context<S> {
    fn default() -> Self {
        Self(None)
    }
}

impl<S: Debug + Clone> Debug for Context<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "Empty"),
            Some(_) => write!(f, "Element()")
        }
    }
}

impl<T: Token + Debug + 'static> Context<BinaryTree<T>> {
    fn push(self: Self, element: BinaryTree<T>) -> Self {
        Context(
            Some(Rc::new(
                move || (element.clone(), self.clone())
            ))
        )
    }

    fn shift_reduce0(
        self: Self,
        tree: BinaryTree<T>,
        grammar: &Grammar<T>,
        mut acc: Vec<Self>) -> Vec<Self> {
        self.0.map_or(
            acc.clone(), // Empty context --> return accumulated contexts
            |ref f| {
                let (popped, rest) = f();
                grammar
                .lookup_nonterminals(&(popped.label(), tree.label()))
                .map_or(
                    // No matching nonterminals --> return accumulated contexts
                    acc.clone(),
                    |new_nonterminal_labels| {
                        new_nonterminal_labels.iter()
                        .flat_map(|new_nonterminal_label| {
                            let new_nonterminal = BinaryTree::Nonterminal {
                                label: new_nonterminal_label.clone(),
                                left: Box::new(popped.clone()),
                                right: Box::new(tree.clone())
                            };
                            acc.push(
                                rest
                                .clone()
                                .push(new_nonterminal.clone())
                            );
                            rest.clone().shift_reduce0(
                                new_nonterminal,
                                grammar,
                                acc.clone()
                            )
                        })
                        .collect()
                    }
                )
            }
        )
    }

    pub fn shift_reduce(self: Self, tree: BinaryTree<T>, grammar: &Grammar<T>)
    -> Vec<Self> {
        self
        .clone()
        .shift_reduce0(tree.clone(), grammar, vec![self.push(tree)])
    }
}

impl Display for Context<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
        let x = context.shift_reduce(BinaryTree::Terminal{ label: "E".to_string(), token: '1' }, &Grammar::expression());
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
                    label: "E".to_string(),
                    token: 'b'
                },
                Context(Some(
                    Rc::new(
                        || (
                            BinaryTree::Terminal{
                                label: "E".to_string(),
                                token: 'a'
                            },
                            Context(None)
                        )
                    )
                ))
            ))
        ));
        let x = context.shift_reduce( BinaryTree::Terminal{ label: "E".to_string(), token: 'c' }, &Grammar::expression());
        assert_eq!(1, x.len(), "{x:?}");
    }
}