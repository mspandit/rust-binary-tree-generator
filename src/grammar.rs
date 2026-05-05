use std::collections::HashMap;

use crate::{Token, binary_tree::BinaryTree};

pub struct Grammar<T: Token, S> {
    unigrams: HashMap<T, Vec<S>>, // token -> terminal labels
    // (left_label, right_label) -> nonterminal labels
    digrams: HashMap<(S, S), Vec<S>>,
}

impl<T: Token> Grammar<T, BinaryTree<T>> {
    pub fn apply(&self, token: & T) -> Vec<BinaryTree<T>> {
        self.unigrams.get(token).iter()
        .flat_map(|terminals|
            terminals.iter().map(|terminal| BinaryTree::Terminal {
                label: terminal.label(),
                token: token.clone()
            }
        ))
        .collect()
    }

    pub fn apply_partial(&self, lr: (BinaryTree<T>, BinaryTree<T>)) -> Vec<BinaryTree<T>> {
        self.digrams.get(& (lr.0.clone(), lr.1.clone())).iter()
        .flat_map(|nonterminals|
            nonterminals.iter().map(|nonterminal| BinaryTree::Nonterminal {
                label: nonterminal.label(),
                left: Box::new(lr.0.clone()),
                right: Box::new(lr.1.clone())
            }
        ))
        .collect()
    }
}

impl Grammar<char, BinaryTree<char>> {
    pub fn expression() -> Self {
        let mut unigrams = HashMap::default();
        unigrams.insert('1', vec![BinaryTree::Terminal { label: "E".to_string(), token: '1' }]);
        unigrams.insert('2', vec![BinaryTree::Terminal { label: "E".to_string(), token: '2' }]);
        unigrams.insert('3', vec![BinaryTree::Terminal { label: "E".to_string(), token: '3' }]);
        unigrams.insert('4', vec![BinaryTree::Terminal { label: "E".to_string(), token: '4' }]);
        unigrams.insert('-', vec![
            BinaryTree::Nonterminal { label: "UnOp".to_string(), left: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: '-' }), right: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }) },
            BinaryTree::Nonterminal { label: "BinOp".to_string(), left: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }), right: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }) }
        ]);
        unigrams.insert('+', vec![BinaryTree::Nonterminal { label: "BinOp".to_string(), left: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }), right: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }) }]);
        unigrams.insert('*', vec![BinaryTree::Nonterminal { label: "BinOp".to_string(), left: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }), right: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }) }]);
        let mut digrams = HashMap::default();
        digrams.insert((BinaryTree::Nonterminal { label: "UnOp".to_string(), left: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: '-' }), right: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }) }, BinaryTree::Terminal { label: "E".to_string(), token: 'E' }), vec![
            BinaryTree::Terminal { label: "E".to_string(), token: 'E' }
        ]);
        digrams.insert((BinaryTree::Terminal { label: "E".to_string(), token: 'E' }, BinaryTree::Nonterminal { label: "BinOp".to_string(), left: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }), right: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }) }), vec![
            BinaryTree::Nonterminal { label: "EBO".to_string(), left: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }), right: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }) }
        ]);
        digrams.insert((BinaryTree::Nonterminal { label: "EBO".to_string(), left: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }), right: Box::new(BinaryTree::Terminal { label: "E".to_string(), token: 'E' }) }, BinaryTree::Terminal { label: "E".to_string(), token: 'E' }), vec![
            BinaryTree::Terminal { label: "E".to_string(), token: 'E' }
        ]);
        Self { unigrams, digrams }
    }
}

impl Grammar<&str, BinaryTree<&str>> {
    pub fn sentence() -> Self {
        let mut unigrams = HashMap::default();
        unigrams.insert("the", vec![BinaryTree::Terminal { label: "Det".to_string(), token: "the" }]);
        unigrams.insert("cat", vec![BinaryTree::Terminal { label: "N".to_string(), token: "cat" }]);
        unigrams.insert("sat", vec![BinaryTree::Terminal { label: "V".to_string(), token: "sat" }]);
        unigrams.insert("on", vec![BinaryTree::Terminal { label: "P".to_string(), token: "on" }]);
        unigrams.insert("mat", vec![BinaryTree::Terminal { label: "N".to_string(), token: "mat" }]);
        let mut digrams = HashMap::default();
        digrams.insert((BinaryTree::Terminal { label: "Det".to_string(), token: "the" }, BinaryTree::Terminal { label: "N".to_string(), token: "cat" }), vec![
            BinaryTree::Nonterminal { label: "NP".to_string(), left: Box::new(BinaryTree::Terminal { label: "Det".to_string(), token: "the" }), right: Box::new(BinaryTree::Terminal { label: "N".to_string(), token: "cat" }) }
        ]);
        digrams.insert((BinaryTree::Terminal { label: "V".to_string(), token: "sat" }, BinaryTree::Nonterminal { label: "NP".to_string(), left: Box::new(BinaryTree::Terminal { label: "Det".to_string(), token: "the" }), right: Box::new(BinaryTree::Terminal { label: "N".to_string(), token: "cat" }) }), vec![
            BinaryTree::Nonterminal { label: "VP".to_string(), left: Box::new(BinaryTree::Terminal { label: "V".to_string(), token: "sat" }), right: Box::new(BinaryTree::Nonterminal { label: "NP".to_string(), left: Box::new(BinaryTree::Terminal { label: "Det".to_string(), token: "the" }), right: Box::new(BinaryTree::Terminal { label: "N".to_string(), token: "cat" }) }) }
        ]);
        digrams.insert((BinaryTree::Terminal { label: "P".to_string(), token: "on" }, BinaryTree::Nonterminal { label: "NP".to_string(), left: Box::new(BinaryTree::Terminal { label: "Det".to_string(), token: "the" }), right: Box::new(BinaryTree::Terminal { label: "N".to_string(), token: "cat" }) }), vec![
            BinaryTree::Nonterminal { label: "PP".to_string(), left: Box::new(BinaryTree::Terminal { label: "P".to_string(), token: "on" }), right: Box::new(BinaryTree::Terminal { label: "N".to_string(), token: "mat" }) }
        ]);
        digrams.insert((BinaryTree::Terminal { label: "V".to_string(), token: "sat" }, BinaryTree::Nonterminal { label: "PP".to_string(), left: Box::new(BinaryTree::Terminal { label: "P".to_string(), token: "on" }), right: Box::new(BinaryTree::Terminal { label: "N".to_string(), token: "mat" }) }), vec![
            BinaryTree::Nonterminal { label: "VP".to_string(), left: Box::new(BinaryTree::Terminal { label: "V".to_string(), token: "sat" }), right: Box::new(BinaryTree::Nonterminal { label: "PP".to_string(), left: Box::new(BinaryTree::Terminal { label: "P".to_string(), token: "on" }), right: Box::new(BinaryTree::Terminal { label: "N".to_string(), token: "mat" }) }) }
        ]);
        Self { unigrams, digrams }
    }
}