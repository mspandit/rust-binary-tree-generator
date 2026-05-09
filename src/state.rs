use std::fmt::Debug;
use crate::{Token, grammar::Grammar, context::Context};

#[derive(Debug)]
pub struct State<S>(Vec<Context<S>>);

impl<S> Default for State<S> {
    fn default() -> Self {
        Self(vec![Context::default()])
    }
}

impl<S: Clone + Debug + 'static> State<S> {
    #[cfg(test)]
    pub fn len(self: & Self) -> usize {
        self.0.len()
    }

    pub fn process<T: Token + Debug>(self: Self, token: & T, grammar: & dyn Grammar<T, S>) -> Self {
        Self(
            self.0.iter().flat_map(|current_context|
                current_context.shift_reduce(token, grammar)
            ).collect()
        )
    }

    pub fn tops<T: Token>(self: Self, grammar: &dyn Grammar<T, S>) -> Vec<S> {
        self.0.into_iter().flat_map(|context| context.0.map_or(
            Vec::default(), // Empty context --> return empty vector
            // Non-empty context --> return vector with element
            |ref f| if grammar.start(f().0) {
                vec![f().0]
            } else {
                Vec::default()
            },
        ))
        .collect()
    }

    pub fn single_contexts(self: Self) -> Self {
        Self({
            let retval = self.0.into_iter().filter(|context| match context.0 {
                None => false, // Filter out empty contexts
                Some(ref f) => match f() {
                    (_, Context(None)) => true,
                    // Filter out contexts with more than one element
                    (_, Context(Some(_))) => false
                }
            })
            .collect();
            retval
        })
    }
}
