use std::{fmt::Debug, rc::Rc};

pub enum Grammar<T, N> {
    Shift(Rc<dyn Fn(& T) -> Vec<Grammar<T, N>>>),
    Reduce(Vec<Grammar<T, N>>),
    Nonterminal(N),
}

impl<T, N> Grammar<T, N>
where T: Clone + 'static + Debug, N: Clone + 'static + Debug {
    pub fn apply(self: & Self, token: & T) -> Vec<Grammar<T, N>> {
        use Grammar::*;
        match self {
            Shift(own_f) => own_f(token),
            Reduce(_) => self.reduce().iter().flat_map(|g| g.apply(token)).collect(),
            Nonterminal(_) => vec![],
        }
    }

    pub fn reduce(self: & Self) -> Vec<Grammar<T, N>> {
        use Grammar::*;
        if let Reduce(v) = self {
            v.iter().flat_map(|g| g.reduce()).collect()
        } else {
            vec![self.clone()]
        }
    }

    // A _choice_ operator. The resulting grammar applies
    // self and other to the input, and returns both results.
    pub fn or(self: Self, other: Self) -> Self {
        use Grammar::*;
        match (self.clone(), other.clone()) {
            (Shift(_), Shift(_)) => Shift(Rc::new(move |input: & T| {
                let mut results = self.apply(input);
                results.extend(other.apply(input));
                results
            })),
            (Shift(_), Reduce(_)) => Reduce({
                let mut results = vec![self.clone()];
                results.extend(other.reduce());
                results
            }),
            (Shift(_), Nonterminal(_)) => Reduce({
                let mut results = vec![self.clone()];
                results.push(other.clone());
                results
            }),
            (Reduce(_), Shift(_)) => Reduce({
                let mut results = self.reduce();
                results.push(other.clone());
                results
            }),
            (Reduce(_), Reduce(_)) => Reduce({
                let mut results = self.reduce();
                results.extend(other.reduce());
                results
            }),
            (Reduce(_), Nonterminal(_)) => todo!(),
            (Nonterminal(_), Shift(_)) =>
                Reduce(vec![self.clone(), other.clone()]),
            (Nonterminal(_), Reduce(_)) => todo!(),
            (Nonterminal(_), Nonterminal(_)) => todo!(),
        }

    }

    // A _sequence_ operator. The resulting grammar applies
    // self to an input to give a list of results.
    //
    // If a result is a Nonterminalinal n, then f(n) returns a
    // grammar to be applied to a subsequent input. It
    // is returned as a ParseResult::Nonterminal.
    //
    // If a result is a Nonterminal g, then g must be
    // applied to the subsequent input first. g.then(f) is
    // returned as a ParseResult::Nonterminal.
    pub fn then<F, U>(self: Self, f: F) -> Grammar<T, U>
    where F: Fn(N) -> Grammar<T, U> + Clone + 'static,
        U: Clone + 'static + Debug {
        use Grammar::*;
        let intermediate_result_fn = move |own_f_res|
            match own_f_res {
                Shift(_) | Reduce(_) => vec![own_f_res.then(f.clone())],
                Nonterminal(n) => {
                    let result = f(n.clone()).reduce();
                    result
                },
            };
        match self {
            Shift(_) => Shift(Rc::new(move |token: & T| {
                self.apply(token).iter()
                .cloned()
                .flat_map(intermediate_result_fn.clone())
                .collect()
            })),
            Reduce(_) => Reduce(
                self.reduce().iter()
                .cloned()
                .flat_map(intermediate_result_fn)
                .collect()
            ),
            Nonterminal(_) => todo!(),
        }
    }

    // Kleene star operator. The resulting grammar applies
    // self to the input zero or more times, returning a
    // vector of the results.
    pub fn star(self: Self) -> Grammar<T, Vec<N>> {
        Grammar::Nonterminal(vec![])
        .or(self.clone().plus())
    }

    // Kleene plus operator. The resulting grammar applies
    // self to the input one or more times, returning a
    // vector of the results.
    pub fn plus(self: Self) -> Grammar<T, Vec<N>> {
        self.clone()
        .then(move |a| self.clone()
            .star()
            .then(move |a_s| {
                let mut result = vec![a.clone()];
                result.extend(a_s);
                Grammar::from(result)
            })
        )
    }

    pub fn parse(self: & Self, input_sequence: & Vec<T>) -> Vec<Grammar<T, N>> {
        use Grammar::*;
        input_sequence.iter().fold(
            match self {
                Nonterminal(_) => vec![self.clone()],
                Shift(_) => vec![self.clone()],
                Reduce(_) => self.reduce(),
            },
            |state, token| {
                state.into_iter()
                .flat_map(|context| match context {
                    Nonterminal(_) => vec![],
                    Shift(_) => context.apply(token),
                    Reduce(_) => context.reduce(),
                })
                .collect()
            }
        )
    }
}

impl<T, N> Clone for Grammar<T, N>
where N: Clone {
    fn clone(&self) -> Self {
        match self {
            Self::Nonterminal(n) => Self::Nonterminal(n.clone()),
            Self::Shift(f) => Self::Shift(f.clone()),
            Self::Reduce(f) => Self::Reduce(f.clone()),
        }
    }
}

impl<T, N> Debug for Grammar<T, N>
where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grammar::Shift(_) => write!(f, "Shift"),
            Grammar::Nonterminal(n) => write!(f, "Nonterminal({n:?})"),
            Grammar::Reduce(v) => write!(f, "Reduce({v:?})"),
        }
    }
}

// Implements the monadic return operator, returning a grammar
// that returns the given value without consuming any of the
// input.
impl<T, N> From<N> for Grammar<T, N>
where N: Clone + 'static + Debug {
    fn from(value: N) -> Self {
        Grammar::Nonterminal(value.clone())
    }
}

fn recurse<T: 'static, N: 'static>(f: &'static dyn Fn(& Grammar<T, N>, & T) -> Vec<Grammar<T, N>>, x: & T) -> Vec<Grammar<T, N>> {
    f(& Grammar::Shift(Rc::new(|y: & T| recurse(f, y))), x)
}

pub fn recursive<T: 'static, N: 'static>(f: &'static dyn Fn(& Grammar<T, N>, & T) -> Vec<Grammar<T, N>>) -> Grammar<T, N> {
    Grammar::Shift(Rc::new(move |x: &T| recurse(f, x)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_grammar1() {
        let g: Grammar<char, char> = Grammar::Nonterminal('a');
        let x = g.parse(&vec![]);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('a')));
    }

    #[test]
    fn test_simple_grammar2() {
        let g: Grammar<char, char> = Grammar::Nonterminal('b');
        let x = g.parse(&vec![]);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('b')));
    }

    #[test]
    fn test_simple_grammar3() {
        let g: Grammar<char, char> = Grammar::Shift(Rc::new(|t| if 'c' == *t {
            vec![Grammar::Nonterminal('c')]
        } else {
            vec![]
        }));
        let x = g.parse(&vec!['c']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('c')));
    }
    #[test]
    fn test_from() {
        let x: Vec<Grammar<char, Vec<char>>> = Grammar::Nonterminal(vec![])
        .parse(& vec![]);
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal([])]"
        )
    }
}