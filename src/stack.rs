use std::fmt::Display;

use crate::{Token, binary_tree::BinaryTree, grammar::Grammar};

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

    pub fn shift_reduce(mut self: Self, tree: BinaryTree<T>, grammar: & Grammar<T>)
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
                let new_stacks = vec![self.clone()];

                match grammar.lookup_nonterminals(&(popped_tree.label(), tree.label())) {
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
                                new_stacks.append(&mut r.clone().shift_reduce(new_nonterminal, grammar));
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
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reduce_second_character() {
        let stack = Stack(vec![BinaryTree::Terminal{ label: "UnOp".to_string(), token: '-' }]);
        let x = stack.shift_reduce(BinaryTree::Terminal{ label: "E".to_string(), token: '1' }, &Grammar::expression());
        println!("{x:?}");
        match x[1].0[0] {
            BinaryTree::Terminal { label: _, token: _, } => panic!("Expected a nonterminal, got {x:?}"),
            BinaryTree::Nonterminal { label: _, left: _, right: _ } => (),
        }
    }

    #[test]
    fn test_reduce_third_character() {
        let stack = Stack(vec![
            BinaryTree::Terminal{ label: "E".to_string(), token: 'b' },
            BinaryTree::Terminal{ label: "E".to_string(), token: 'a' },
        ]);
        let x = stack.shift_reduce( BinaryTree::Terminal{ label: "E".to_string(), token: 'c' }, &Grammar::expression());
        assert_eq!(3, x[0].len(), "{x:?}");
    }
}