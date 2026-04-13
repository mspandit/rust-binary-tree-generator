use std::fmt::Display;

use crate::Token;

#[derive(Debug, Clone)]
pub enum BinaryTree<T: Token> {
    Terminal(T),
    Nonterminal(Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}

impl<T: Token> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryTree::Terminal(c) => write!(f, "{c}"),
            BinaryTree::Nonterminal(left, right) =>
                write!(f, "({left} {right})"),
        }
    }
}
