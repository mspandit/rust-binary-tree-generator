use std::{
    fmt::Debug,
    rc::Rc,
};

#[derive(Clone)]
pub enum Grammar<T, N>
where
    N: Clone,
    T: Clone,
{
    Nonterminal(N),
    Reduce(Vec<Grammar<T, N>>),
    Shift(Rc<dyn Fn(&T) -> Grammar<T, N>>),
}

impl<T, N> Grammar<T, N>
where
    N: Clone,
    T: Clone,
{
    // For Nonterminal, fail
    // For Reduce, return a Reduce of the reduction
    // For Shift, call the function
    pub fn shift(self: &Self, t: &T) -> Grammar<T, N>
    where
        N: 'static,
        T: 'static,
    {
        use Grammar::*;
        match self {
            Nonterminal(_) => Reduce(vec![]),
            Reduce(_) => Reduce(self
                .reduce()
                .iter()
                .map(
                    |g| g.shift(t)
                )
                .collect()),
            Shift(ndnary) => ndnary(t),
        }
    }

    // Flatten the grammar to contain only Nonterminal
    // or Shift variants
    pub fn reduce(self: &Self) -> Vec<Self> {
        use Grammar::*;
        match self {
            Reduce(rs) => rs.iter().flat_map(Grammar::reduce).collect(),
            Nonterminal(_) | Shift(_) => vec![self.clone()],
        }
    }

    pub fn or(self: & Self, other: & Self) -> Self {
        Grammar::Reduce(vec![self.clone(), other.clone()])
    }

    pub fn then<M, F>(self: & Self, f: F) -> Grammar<T, M>
    where
        T: 'static + Debug,
        N: 'static + Debug ,
        M: Clone + Debug,
        F: Fn(&N) -> Grammar<T, M> + Clone + 'static,
    {
        use Grammar::*;
        match self.clone() {
            Nonterminal(n) => f(&n),
            Reduce(rs) => {
                let rs = rs.clone();
                Reduce(rs.into_iter().map(|g| g.then(f.clone())).collect())
            },
            Shift(ndnary) => Shift(Rc::new(
                move |t| ndnary(t).then(f.clone())
            )),
        }
    }

    pub fn star(self: Self) -> Grammar<T, Vec<N>>
    where
        T: 'static + Debug,
        N: 'static + Debug,
    {
        use Grammar::*;
        self.clone().plus().or(& Nonterminal(vec![]))
    }

    pub fn plus(self: Self) -> Grammar<T, Vec<N>>
    where
        T: 'static + Debug,
        N: 'static + Debug,
    {
        use Grammar::*;
        self.clone().then(move |a| {
            let a = a.clone();
            self.clone().star().then(move |v_a| {
                let mut result = vec![a.clone()];
                result.extend(v_a.clone());
                Nonterminal(result)
            })
        })
    }

    pub fn parse(self: &Self, inputs: &Vec<T>) -> Vec<Grammar<T, N>>
    where
        T: 'static,
        N: 'static,
    {
        use Grammar::*;
        inputs.iter().fold(
            self.reduce(), // initial reduction
            |state, token| {
                state
                    .into_iter()
                    .flat_map(|context| {
                        context
                            // shift each token, then reduce
                            .shift(token)
                            .reduce()
                    })
                    .collect()
            },
        )
        .into_iter()
        .filter(|t| matches!(t, Nonterminal(_)))
        .collect::<Vec<Grammar<T, N>>>()
    }

    pub fn map<U> (self: & Self, f: fn(& N) -> U) -> Vec<U> {
        use Grammar::*;
        match self {
            Nonterminal(n) => vec![f(n)],
            Reduce(v) => v.iter()
            .flat_map(|g| g.map(f))
            .collect(),
            Shift(_) => vec![],
        }
    }
}

impl<T, N> Debug for Grammar<T, N>
where
    N: Clone + Debug,
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Grammar::*;
        match self {
            Nonterminal(n) => write!(f, "Nonterminal({:?})", n),
            Reduce(rs) => write!(f, "Reduce({:?})", rs),
            Shift(_) => write!(f, "Shift"),
        }
    }
}

// Eliminates the Grammar::Shift(Rc::new(...)) boilerplate
pub fn item<T, U>() -> Grammar<T, U>
where
    T: Clone,
    U: Clone + std::convert::From<T>,
{
    Grammar::Shift(Rc::new(|input: &T| Grammar::Nonterminal(input.clone().into())))
}

pub fn left_recursive<T, N>(generator: fn(usize) -> Grammar<T, N>)
-> Grammar<T, N>
where
    T: Clone + 'static + Debug,
    N: Clone + 'static + Debug
{
    item().star() // Stack inputs
    .then(move |cs: & Vec<T>| {
        // Initialize with grammar of the necessary
        // depth, then apply it to history of inputs
        cs.iter().fold(
            generator(cs.len()),
            |g, c| g.shift(c)
        )
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_grammar3() {
        use Grammar::*;
        let g: Grammar<char, char> = Shift(Rc::new(|t| {
            if 'c' == *t {
                Nonterminal('c')
            } else {
                Reduce(vec![])
            }
        }));
        let x = g.parse(&vec!['c']);
        assert_eq!(x.len(), 1);
        assert!(matches!(x[0], Grammar::Nonterminal('c')));
    }

    #[test]
    fn test_failure() {
        use Grammar::*;
        let failure: Grammar<char, char> = Reduce(vec![]);
        assert_eq!(
            format!("{:?}", failure.shift(& 'a').reduce()),
            "[]"
        );
    }

    #[test]
    fn test_or_identity() {
        use Grammar::*;
        let failure: Grammar<char, char> = Reduce(vec![]);
        let g = failure.or(& item());
        assert_eq!(
            format!("{:?}", g.shift(& 'a').reduce()),
            "[Nonterminal('a')]"
        );
    }
}
