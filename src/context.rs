use std::{fmt::Debug, rc::Rc};

use crate::grammar::{Partial, PartialGrammar};

#[derive(Clone)]
pub enum Symbol<N> {
    Complete(N),
    Incomplete(Rc<Partial<N>>),
}

impl<N> Debug for Symbol<N>
where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Complete(s) => write!(f, "Complete({:?})", s),
            Symbol::Incomplete(_) => write!(f, "Incomplete"),
        }
    }
}

#[derive(Clone)]
pub struct Context<N>(pub Option<Rc<dyn Fn() -> (Symbol<N>, Self)>>)
where N: 'static;

impl<N> Context<N>
where N: Clone + Debug {
    pub fn apply<T>(
        self: & Self,
        symbol: Symbol<N>,
        grammar: & PartialGrammar<T, N>
    )
    -> Vec<Self>
    where T: Clone + 'static {
        match self.0 {
            None =>
                // Context with symbol on top
                vec![self.clone().push(symbol)],
            Some(ref f) => {
                let (top, rest) = f();
                match (top, symbol.clone()) {
                    (Symbol::Complete(_), _) => vec![], // No contexts
                    (Symbol::Incomplete(_), Symbol::Incomplete(_)) =>
                        vec![self.clone().push(symbol)], // Context w/symbol on top
                    (Symbol::Incomplete(p), Symbol::Complete(s)) => {
                        let new_ss = p(& s);
                        let contextual_results: Vec<Self> = new_ss.iter()
                        .flat_map(|symbol|
                            // Recurse on rest of context with new symbols
                            rest.clone().apply(
                                Symbol::Complete(symbol.clone()),
                                grammar
                            )
                        ).collect();
                        let recursive_results: Vec<Self> = grammar
                        .apply_binary(& new_ss)
                        .into_iter()
                        .flat_map(|partial|
                            // Recurse on partials started by the new symbols
                            rest.clone().apply(Symbol::Incomplete(partial), grammar)
                        ).collect();
                        [contextual_results, recursive_results].concat()
                    },
                }
            }
        }
    }

    pub fn apply_token<T>(self: Self, token: & T, g: & PartialGrammar<T, N>)
    -> Vec<Self>
    where T: Clone + 'static {
        let unary_symbols = g.apply_unary(token);
        let binary_symbols: Vec<Symbol<N>> = g
        .apply_binary(& unary_symbols)
        .into_iter().map(Symbol::Incomplete)
        .collect();
        let unary_symbols: Vec<Symbol<N>>= unary_symbols.clone()
        .into_iter().map(Symbol::Complete)
        .collect();
        [unary_symbols, binary_symbols].concat()
        .into_iter().flat_map(|symbol| self.apply(symbol, g))
        .collect()
    }
}

impl<N> Default for Context<N> {
    fn default() -> Self {
        Context(None)
    }
}

impl<N> Debug for Context<N>
where N: Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "Empty"),
            Some(g) => write!(f, "Symbol({:?})", g())
        }
    }
}

impl<S> Context<S>
where S: Clone {
    pub fn push(self: Self, symbol: Symbol<S>) -> Self {
        Context(
            Some(Rc::new(
                move || (symbol.clone(), self.clone())
            ))
        )
    }
}
