use std::{fmt::{Debug, Display}, rc::Rc};

use crate::{Token, binary_tree::BinaryTree, grammar::Grammar};

#[derive(Clone)]
pub struct Stack<S: Debug + Clone>(pub Option<Rc<dyn Fn() -> (S, Self)>>);

impl<S: Clone + Debug> Default for Stack<S> {
    fn default() -> Self {
        Self(None)
    }
}

impl<S: Debug + Clone> Debug for Stack<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "Empty"),
            Some(_) => write!(f, "Element()")
        }
    }
}

impl<T: Token + Debug + 'static> Stack<BinaryTree<T>> {
    pub fn top(&self) -> BinaryTree<T> {
        match self.0 {
             None => panic!("Cannot get top of empty stack"),
             Some(ref f) => f().0,
        }
    }

    fn pop(self: Self) -> (BinaryTree<T>, Self) {
        match self.0 {
            None => panic!("Cannot pop from empty stack"),
            Some(ref f) => f(),
        }
    }

    fn push(self: Self, element: BinaryTree<T>) -> Self {
        let closure = move || (element.clone(), self.clone());
        Stack(Some(Rc::new(closure)))
    }

    pub fn shift_reduce(self: Self, mut tree: BinaryTree<T>, grammar: &Grammar<T>) -> Vec<Self> {
        let mut stack = self.clone();
        let mut retval = vec![stack.clone().push(tree.clone())];
        loop {
            match stack.0 {
                None => break,
                Some(ref f) => {
                    let (popped, rest) = f();
                    match grammar.lookup_nonterminals(&(popped.label(), tree.label())) {
                        None => { break },
                        Some(new_nonterminal_label) => {
                            let new_nonterminal = BinaryTree::Nonterminal {
                                label: new_nonterminal_label.clone(),
                                left: Box::new(popped.clone()),
                                right: Box::new(tree.clone())
                            };
                            retval.push(rest.clone().push(new_nonterminal.clone()));
                            tree = new_nonterminal;
                        }
                    }
                    stack = rest;
                }
            }
        }
        retval
    }
}

impl Display for Stack<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reduce_second_character() {
        let stack = Stack(Some(
            Rc::new(
                || (
                    BinaryTree::Terminal {
                        label: "UnOp".to_string(),
                        token: '-',
                    },
                    Stack(None)
                )
            )
        ));
        let x = stack.shift_reduce(BinaryTree::Terminal{ label: "E".to_string(), token: '1' }, &Grammar::expression());
        println!("{x:?}");
        match x[1].top() {
            BinaryTree::Terminal { label: _, token: _, } => panic!("Expected a nonterminal, got {x:?}"),
            BinaryTree::Nonterminal { label: _, left: _, right: _ } => (),
        }
    }

    #[test]
    fn test_reduce_third_character() {
        let stack = Stack(Some(
            Rc::new(|| (
                BinaryTree::Terminal{
                    label: "E".to_string(),
                    token: 'b'
                },
                Stack(Some(
                    Rc::new(
                        || (
                            BinaryTree::Terminal{
                                label: "E".to_string(),
                                token: 'a'
                            },
                            Stack(None)
                        )
                    )
                ))
            ))
        ));
        let x = stack.shift_reduce( BinaryTree::Terminal{ label: "E".to_string(), token: 'c' }, &Grammar::expression());
        assert_eq!(1, x.len(), "{x:?}");
    }
}