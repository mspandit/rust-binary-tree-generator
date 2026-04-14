use std::fmt::Display;

use crate::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryTree<T: Token> {
    Terminal { label: String, token: T },
    Nonterminal {
        label: String,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>
    },
}

impl<T: Token> BinaryTree<T> {
    pub fn label(&self) -> String {
        match self {
            BinaryTree::Terminal { label, .. } => label.clone(),
            BinaryTree::Nonterminal { label, .. } => label.clone(),
        }
    }
}

impl<T: Token> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryTree::Terminal { label: _, token } => write!(f, "{token}"),
            BinaryTree::Nonterminal { label: _, left, right } =>
                write!(f, "({left} {right})"),
        }
    }
}
