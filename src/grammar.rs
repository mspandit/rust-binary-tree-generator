use std::collections::HashMap;

use crate::{Token, binary_tree::BinaryTree};

pub struct Grammar<T: Token> {
    unigrams: HashMap<T, BinaryTree<T>>,
    digrams: HashMap<(BinaryTree<T>, BinaryTree<T>), BinaryTree<T>>,
}

impl<T: Token> Grammar<T> {
    pub fn lookup_terminal(&self, token: T) -> Option<BinaryTree<T>> {
        self.unigrams.get(&token).cloned()
    }

    pub fn lookup_digram(&self, left: BinaryTree<T>, right: BinaryTree<T>) -> Option<BinaryTree<T>> {
        self.digrams.get(&(left, right)).cloned()
    }
}