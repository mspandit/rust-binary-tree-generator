use std::collections::HashMap;

use crate::Token;

pub struct Grammar<T: Token> {
    unigrams: HashMap<T, Vec<String>>, // token -> terminal label
    // (left_label, right_label) -> nonterminal label
    digrams: HashMap<(String, String), String>,
}

impl<T: Token> Grammar<T> {
    pub fn lookup_terminals(&self, token: & T) -> Option<Vec<String>> {
        self.unigrams.get(token).cloned()
    }

    pub fn lookup_nonterminals(&self, lr: &(String, String)) -> Option<String> {
        self.digrams.get(&lr).cloned()
    }
}

impl Grammar<char> {
    pub fn expression() -> Self {
        let mut unigrams = HashMap::default();
        unigrams.insert('1', vec!["E".to_string()]);
        unigrams.insert('2', vec!["E".to_string()]);
        unigrams.insert('3', vec!["E".to_string()]);
        unigrams.insert('4', vec!["E".to_string()]);
        unigrams.insert('-', vec![
            "UnOp".to_string(),
            "BinOp".to_string()
        ]);
        unigrams.insert('+', vec!["BinOp".to_string()]);
        unigrams.insert('*', vec!["BinOp".to_string()]);
        let mut digrams = HashMap::default();
        digrams.insert(("UnOp".to_string(), "E".to_string()), "E".to_string());
        digrams.insert(("E".to_string(), "BinOp".to_string()), "EBO".to_string());
        digrams.insert(("EBO".to_string(), "E".to_string()), "E".to_string());
        Self { unigrams, digrams }
    }
}

impl Grammar<&str> {
    pub fn sentence() -> Self {
        let mut unigrams = HashMap::default();
        unigrams.insert("the", vec!["Det".to_string()]);
        unigrams.insert("cat", vec!["N".to_string()]);
        unigrams.insert("sat", vec!["V".to_string()]);
        unigrams.insert("on", vec!["P".to_string()]);
        unigrams.insert("mat", vec!["N".to_string()]);
        let mut digrams = HashMap::default();
        digrams.insert(("Det".to_string(), "N".to_string()), "NP".to_string());
        digrams.insert(("V".to_string(), "NP".to_string()), "VP".to_string());
        digrams.insert(("P".to_string(), "NP".to_string()), "PP".to_string());
        digrams.insert(("V".to_string(), "PP".to_string()), "VP".to_string());
        digrams.insert(("NP".to_string(), "VP".to_string()), "S".to_string());
        Self { unigrams, digrams }
    }
}