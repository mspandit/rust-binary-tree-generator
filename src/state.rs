use std::fmt::Debug;
use crate::{context::{Context, Symbol}, grammar::{Grammar, PartialGrammar, Start}};

#[derive(Debug)]
pub struct State<T, N>
where N: 'static {
    context: Vec<Context<N>>,
    grammar: PartialGrammar<T, N>,
}

impl<T, N> State<T, N>
where T: Debug + Clone + 'static, N: Start<N> + Clone + Debug + 'static {
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.context.len()
    }

    pub fn new(grammar: Grammar<T, N>) -> Self {
        Self {
            context: vec![Context::default()],
            grammar: PartialGrammar(grammar),
        }
    }

    pub fn apply(self: Self, token: & T) -> Self {
        Self {
            context: self.context.into_iter().flat_map(|current_context|
                current_context.apply_token(token, &self.grammar)
            ).collect(),
            grammar: self.grammar,
        }
    }

    pub fn tops(self: Self) -> Vec<N> {
        self.context.into_iter().flat_map(|context| context.0.map_or(
            Vec::default(), // Empty context --> return empty vector
            // Non-empty context --> return vector with symbol
            |ref f| match f().0 {
                Symbol::Complete(s) if s.start(s.clone()) => vec![s],
                _ => Vec::default(),
            },
        ))
        .collect()
    }

    pub fn single_contexts(self: Self) -> Self {
        Self {
            context: self.context.into_iter().filter(|context| match context.0 {
                None => false, // Filter out empty contexts
                Some(ref f) => match f() {
                    (_, Context(None)) => true,
                    // Filter out contexts with more than one symbol
                    (_, Context(Some(_))) => false
                }
            })
            .collect(),
            grammar: self.grammar,
        }
    }
}
