use std::fmt::Debug;
use crate::{Token, binary_tree::BinaryTree, grammar::Grammar, stack::Stack};

#[derive(Debug)]
pub struct GeneratorState<T: Token + Debug>(Vec<Stack<BinaryTree<T>>>);

impl<T: Token + Debug> Default for GeneratorState<T> {
    fn default() -> Self {
        Self(vec![Stack::default()])
    }
}

impl<T: Token + Debug + 'static> GeneratorState<T> {
    #[cfg(test)]
    pub fn len(self: & Self) -> usize {
        self.0.len()
    }

    pub fn process(self: Self, token: T, grammar: &Grammar<T>) -> Self {
        Self(
            self.0.iter().flat_map(|current_stack| {
                grammar.lookup_terminals(& token).map_or(
                    Vec::default(),
                    |t_labels| {
                        t_labels.iter().flat_map(|terminal_label| {
                            current_stack.clone()
                                .shift_reduce(
                                    BinaryTree::Terminal {
                                        label: terminal_label.clone(),
                                        token: token.clone()
                                    },
                                    grammar
                                )
                        }).collect()
                    }
                )
            }).collect()
        )
    }

    pub fn tops(self: Self) -> Vec<BinaryTree<T>> {
        self.0.into_iter().flat_map(|stack| stack.0.map_or(
            Vec::default(), // Empty stack --> return empty vector
            // Non-empty stack --> return vector with element
            |ref f| vec![f().0]
        ))
        .collect()
    }

    pub fn filter_stacks(self: Self) -> Self {
        Self(
            self.0.into_iter().filter(|stack| match stack.0 {
                None => false, // Filter out empty stacks
                Some(ref f) => match f() {
                    (_, Stack(None)) => true,
                    // Filter out stacks with more than one element
                    (_, Stack(Some(_))) => false
                }
            })
            .collect()
        )
    }
}
