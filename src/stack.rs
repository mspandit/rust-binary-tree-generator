use std::fmt::Display;

use crate::{Token, binary_tree::BinaryTree};

#[derive(Debug, Clone, Default)]
pub struct Stack<T: Token>(Vec<BinaryTree<T>>);

impl<T: Token> Stack<T> {
    pub fn top(&self) -> BinaryTree<T> {
        self.0[0].clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn pop(&mut self) -> Option<BinaryTree<T>> {
        self.0.pop()
    }

    fn push(&mut self, tree: BinaryTree<T>) {
        self.0.push(tree);
    }

    pub fn shift_reduce(mut self: Self, tree: BinaryTree<T>)
    -> Vec<Self> {
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
                let mut new_stacks = vec![self.clone()];

                let new_nonterminal = BinaryTree::Nonterminal(
                    Box::new(popped_tree),
                    Box::new(tree),
                );
                new_stacks.append(&mut r
                    .shift_reduce(new_nonterminal)
                ); // reduce
                new_stacks
            }
        }
    }
}

impl Display for Stack<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reduce_second_character() {
        let stack = Stack(vec![BinaryTree::Terminal('a')]);
        let x = stack.shift_reduce( BinaryTree::Terminal('b'));
        match x[1].0[0] {
            BinaryTree::Nonterminal(_, _) => (),
            _ => panic!("Expected a nonterminal, got {x:?}"),
        }
    }

    #[test]
    fn test_reduce_third_character() {
        let stack = Stack(vec![
            BinaryTree::Terminal('b'),
            BinaryTree::Terminal('a'),
        ]);
        let x = stack.shift_reduce( BinaryTree::Terminal('c'));
        assert_eq!(3, x[0].len(), "{x:?}");
    }
}