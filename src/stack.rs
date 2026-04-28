use std::{fmt::{Debug, Display}, rc::Rc};

use crate::{Token, binary_tree::BinaryTree, grammar::Grammar};

#[derive(Clone, Default)]
pub enum Stack<T: Token + Debug> {
    #[default]
    Empty,
    Element(BinaryTree<T>, Rc<dyn Fn() -> Stack<T>>)
}

impl<T: Token + Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stack::Empty => write!(f, "Empty"),
            Stack::Element(tree, _) => write!(f, "Element({:?})", tree)
        }
    }
}

impl<T: Token + Debug> Stack<T> {
    pub fn top(&self) -> BinaryTree<T> {
        match self {
            Stack::Empty => panic!("Cannot get top of empty stack"),
            Stack::Element(tree, _) => tree.clone()
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Stack::Empty => 0,
            Stack::Element(_, _) => 1
        }
    }

    fn pop(&mut self) -> Option<BinaryTree<T>> {
        match self {
            Stack::Empty => None,
            Stack::Element(tree, rest) => {
                let retval = Some(tree.clone());
                *self = rest();
                retval
            }
        }
    }

    fn push(&mut self, tree: BinaryTree<T>) {
        match self {
            Stack::Empty => *self = Stack::Element(tree, Rc::new(|| Stack::Empty)),
            Stack::Element(_, next_stack_fn) => {
                *self = Stack::Element(tree, next_stack_fn.clone());
            }
        }
    }

    pub fn shift_reduce(
        mut self: Self,
        tree: BinaryTree<T>,
        grammar: & Grammar<T>
    ) -> Vec<Self> {
        match self.pop() {
            None => {
                self.push(tree); // shift only
                vec![self]
            },
            Some(popped_tree) => {
                let r = self.clone(); // for recursion later
                // restore the popped tree
                self.push(popped_tree.clone());
                self.push(tree.clone()); // shift
                let new_stacks = vec![self.clone()];

                match grammar.lookup_nonterminals(
                    &(popped_tree.label(), tree.label())
                ) {
                    None => new_stacks,
                    Some(new_nonterminal_labels) => {
                        new_nonterminal_labels.iter().fold(
                            new_stacks,
                            |mut new_stacks, new_nonterminal_label| {
                                let new_nonterminal = BinaryTree::Nonterminal {
                                    label: new_nonterminal_label.clone(),
                                    left: Box::new(popped_tree.clone()),
                                    right: Box::new(tree.clone()),
                                };
                                new_stacks.append(&mut r.clone()
                                    .shift_reduce(new_nonterminal, grammar)
                                );
                                new_stacks
                            }
                        )
                    }
                }
            }
        }
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
        let stack = Stack::Element(BinaryTree::Terminal { label: "UnOp".to_string(), token: '-' }, Rc::new(|| Stack::Empty));
        let x = stack.shift_reduce(BinaryTree::Terminal{ label: "E".to_string(), token: '1' }, &Grammar::expression());
        println!("{x:?}");
        match x[1].top() {
            BinaryTree::Terminal { label: _, token: _, } => panic!("Expected a nonterminal, got {x:?}"),
            BinaryTree::Nonterminal { label: _, left: _, right: _ } => (),
        }
    }

    #[test]
    fn test_reduce_third_character() {
        let stack = Stack::Element(BinaryTree::Terminal{ label: "E".to_string(), token: 'b'}, Rc::new(|| Stack::Element(BinaryTree::Terminal{ label: "E".to_string(), token: 'a'}, Rc::new(|| Stack::Empty))));
        let x = stack.shift_reduce( BinaryTree::Terminal{ label: "E".to_string(), token: 'c' }, &Grammar::expression());
        assert_eq!(3, x[0].len(), "{x:?}");
    }
}