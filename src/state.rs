use std::fmt::Debug;
use crate::{Token, binary_tree::BinaryTree, grammar::Grammar, context::Context};

#[derive(Debug)]
pub struct State<T: Token + Debug>(Vec<Context<BinaryTree<T>>>);

impl<T: Token + Debug> Default for State<T> {
    fn default() -> Self {
        Self(vec![Context::default()])
    }
}

impl<T: Token + Debug + 'static> State<T> {
    #[cfg(test)]
    pub fn len(self: & Self) -> usize {
        self.0.len()
    }

    pub fn process(self: Self, token: T, grammar: &Grammar<T>) -> Self {
        Self(
            self.0.iter().flat_map(|current_context| {
                current_context.shift_reduce(&token, grammar)
            }).collect()
        )
    }

    pub fn tops(self: Self) -> Vec<BinaryTree<T>> {
        self.0.into_iter().flat_map(|context| context.0.map_or(
            Vec::default(), // Empty context --> return empty vector
            // Non-empty context --> return vector with element
            |ref f| vec![f().0]
        ))
        .collect()
    }

    pub fn filter_contexts(self: Self) -> Self {
        Self(
            self.0.into_iter().filter(|context| match context.0 {
                None => false, // Filter out empty contexts
                Some(ref f) => match f() {
                    (_, Context(None)) => true,
                    // Filter out contexts with more than one element
                    (_, Context(Some(_))) => false
                }
            })
            .collect()
        )
    }
}
