use std::{fmt::Debug, rc::Rc};

use crate::{Token, grammar::Grammar};

#[derive(Clone)]
pub struct Context<S>(pub Option<Rc<dyn Fn() -> (S, Self)>>);

impl<S> Default for Context<S> {
    fn default() -> Self {
        Self(None)
    }
}

impl<S: Debug> Debug for Context<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "Empty"),
            Some(g) => write!(f, "Element({:?})", g())
        }
    }
}

impl<S: Clone + 'static> Context<S> {
    pub fn push(self: Self, element: S) -> Self {
        Context(
            Some(Rc::new(
                move || (element.clone(), self.clone())
            ))
        )
    }
}

impl<S: Clone + Debug + 'static> Context<S> {
    pub fn shift_reduce<T: Token>(self: & Self, token: & T, grammar: &dyn Grammar<T, S>)
    -> Vec<Self> {
        grammar.apply(token).iter().flat_map(|symbol| {
            grammar.apply_contextual(self.clone(), symbol.clone())
        })
        .collect()
    }
}