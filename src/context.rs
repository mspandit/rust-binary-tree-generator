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

    pub fn shift_reduce(self: & Self, token: & T, grammar: &Grammar<T>)
    -> Vec<Self> {
        grammar.lookup_terminals(& token).map_or(
            Vec::default(),
            |t_labels| {
                t_labels.iter().flat_map(|terminal_label| {
                    let tree = BinaryTree::Terminal {
                        label: terminal_label.clone(),
                        token: token.clone()
                    };
                    let acc = vec![self.clone().push(tree.clone())];
                    self.clone().shift_reduce0(tree, grammar, acc)
                })
                .collect()
            }
        )
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