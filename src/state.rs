use std::{fmt::Debug, hash::Hash};
use crate::{Token, grammar::Grammar, context::Context};

#[derive(Debug)]
pub struct State<S>(Vec<Context<S>>);

impl<S> Default for State<S> {
    fn default() -> Self {
        Self(vec![Context::default()])
    }
}

impl<S: Clone + Eq + Hash + 'static + Debug> State<S> {
    #[cfg(test)]
    pub fn len(self: & Self) -> usize {
        self.0.len()
    }

    pub fn process<T: Token>(self: Self, token: T, grammar: & dyn Grammar<T, S>) -> Self {
        Self(
            self.0.iter().flat_map(|current_context| {
                current_context.shift_reduce(&token, grammar)
            }).collect()
        )
    }

    pub fn tops<T: Token>(self: Self) -> Vec<S> {
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
