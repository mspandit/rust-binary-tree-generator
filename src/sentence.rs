use std::{rc::Rc, fmt::Debug};

use crate::grammar::Grammar;


#[derive(Clone)]
pub enum Sentence {
    Det(String),
    N(String),
    P(String),
    V(String),
    NP(String),
    PP(String),
    VP(String),
    S(String),
}

pub fn noun() -> Grammar<String, Sentence> {
    let cat = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "cat" {
            vec![Grammar::Nonterminal(Sentence::N(token.clone()))]
        } else {
            vec![]
        }
    ));
    let mat = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "mat" {
            vec![Grammar::Nonterminal(Sentence::N(token.clone()))]
        } else {
            vec![]
        }
    ));
    cat.or(mat)
}

pub fn noun_phrase() -> Grammar<String, Sentence> {
    let det = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "the" {
            vec![Grammar::Nonterminal(Sentence::Det(token.clone()))]
        } else {
            vec![]
        }
    ));
    det.then(
        move |d| {
            let d = d.clone();
            noun().then(move |n_sym| {
                Grammar::from(Sentence::NP(format!("({:?} {:?})", d.clone(), n_sym.clone())))
            })
        }
    )
}

pub fn sentence() -> Grammar<String, Sentence> {
    let v = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "sat" {
            vec![Grammar::Nonterminal(Sentence::V(token.clone()))]
        } else {
            vec![]
        }
    ));
    let p = Grammar::Shift(Rc::new(|token: & String|
        if token.as_str() == "on" {
            vec![Grammar::Nonterminal(Sentence::P(token.clone()))]
        } else {
            vec![]
        }
    ));
    let np = noun_phrase();

    let pp = p.clone().then({
        let np = np.clone();
        move |p_sym| {
            let p_sym = p_sym.clone();
            np.clone().then(move |np_sym| {
                Grammar::from(Sentence::PP(format!("({:?} {:?})", p_sym.clone(), np_sym.clone())))
            })
        }
    });

    let vp = v.clone()
    .then({
        let np = np.clone();
        move |v_sym| {
            let v_sym = v_sym.clone();
            np.clone().then(move |np_sym| {
                Grammar::from(Sentence::VP(format!("({:?} {:?})", v_sym.clone(), np_sym.clone())))
            })
        }
    })
    .or(v.then({
        let pp = pp.clone();
        move |v_sym| {
            let v_sym = v_sym.clone();
            pp.clone().then(move |pp_sym| {
                Grammar::from(Sentence::VP(format!("({:?} {:?})", v_sym.clone(), pp_sym.clone())))
            })
        }
    }));

    np.then(move |np_sym| {
        let np_sym = np_sym.clone();
        vp.clone().then(move |vp_sym| {
            Grammar::from(Sentence::S(format!("({:?} {:?})", np_sym.clone(), vp_sym.clone())))
        })
    })
}

impl Debug for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sentence::Det(s) | Sentence::N(s) | Sentence::P(s) | Sentence::V(s) | Sentence::NP(s) | Sentence::PP(s) | Sentence::VP(s) | Sentence::S(s) => write!(f, "{}", s)
        }
    }
}

#[cfg(test)]
mod test {

use super::*;

    #[test]
    fn test_one_word() {
        let input = vec!["the".to_string()];
        let x = sentence()
        .parse(& input);
        assert_eq!(
             format!("{:?}", x),
            "[Shift]",
        );
    }

    #[test]
    fn test_two_words() {
        let input = vec!["the".to_string(), "cat".to_string()];
        let x = sentence().parse(&input);
        assert_eq!("[Shift]", format!("{:?}", x));
    }

    #[test]
    fn test_six_words() {
        let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let x = sentence().parse(&word_sequence);
        assert_eq!(1, x.len(), "{x:?}");
        assert_eq!(
            format!("{:?}", x),
            "[Nonterminal(((the cat) (sat (on (the mat)))))]",
        )
    }

}