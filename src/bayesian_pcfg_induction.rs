// Lifeng Jin, Lane Schwartz, Finale Doshi-Velez, Timothy Miller, William Schuler; Depth-Bounded Statistical PCFG Induction as a Model of Human Grammar Acquisition. Computational Linguistics 2021; 47 (1): 181–216. doi: https://doi.org/10.1162/coli_a_00399

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    N(String),
    T(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub lhs: String,
    pub rhs: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub rules: Vec<Rule>,
    pub alpha: f64,
}

#[derive(Debug, Clone)]
pub struct ParseTree {
    pub root: String,
    pub rules_used: Vec<Rule>,
    pub yield_tokens: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ModelState {
    pub grammar: Grammar,
    pub parses: Vec<ParseTree>,
    pub counts: HashMap<Rule, usize>,
}

impl ModelState {
    pub fn new(grammar: Grammar, parses: Vec<ParseTree>) -> Self {
        let mut counts = HashMap::new();
        for p in &parses {
            for r in &p.rules_used {
                *counts.entry(r.clone()).or_insert(0) += 1;
            }
        }
        Self { grammar, parses, counts }
    }

    pub fn resample_parses(&mut self) {
        for i in 0..self.parses.len() {
            let old = self.parses[i].clone();
            self.decrement_counts(&old.rules_used);
            let new_parse = self.sample_parse_for_sentence(&old.yield_tokens);
            self.increment_counts(&new_parse.rules_used);
            self.parses[i] = new_parse;
        }
    }

    pub fn update_rule_posteriors(&self) -> HashMap<Rule, f64> {
        let mut totals_by_lhs: HashMap<String, usize> = HashMap::new();
        for r in &self.grammar.rules {
            let c = self.counts.get(r).copied().unwrap_or(0);
            *totals_by_lhs.entry(r.lhs.clone()).or_insert(0) += c;
        }

        let mut post = HashMap::new();
        for r in &self.grammar.rules {
            let c_r = self.counts.get(r).copied().unwrap_or(0) as f64;
            let lhs_total = totals_by_lhs.get(&r.lhs).copied().unwrap_or(0) as f64;
            let k = self.grammar.rules.iter().filter(|x| x.lhs == r.lhs).count() as f64;
            let p = (c_r + self.grammar.alpha) / (lhs_total + k * self.grammar.alpha);
            post.insert(r.clone(), p);
        }
        post
    }

    fn sample_parse_for_sentence(&self, tokens: &[String]) -> ParseTree {
        let start = "S".to_string();
        let mut rules_used = Vec::new();

        if tokens.len() == 1 {
            let rule = self
                .grammar
                .rules
                .iter()
                .find(|r| r.lhs == start && r.rhs == vec![Symbol::T(tokens[0].clone())])
                .expect("No unary terminal rule for sentence");
            rules_used.push(rule.clone());
        } else if tokens.len() == 2 {
            let rule = self
                .grammar
                .rules
                .iter()
                .find(|r| {
                    r.lhs == start
                        && r.rhs.len() == 2
                        && matches!(&r.rhs[0], Symbol::T(_))
                        && matches!(&r.rhs[1], Symbol::T(_))
                })
                .expect("No binary terminal rule for sentence");
            rules_used.push(rule.clone());
        } else {
            panic!("Toy sampler only supports sentences of length 1 or 2");
        }

        ParseTree {
            root: start,
            rules_used,
            yield_tokens: tokens.to_vec(),
        }
    }

    fn increment_counts(&mut self, rules: &[Rule]) {
        for r in rules {
            *self.counts.entry(r.clone()).or_insert(0) += 1;
        }
    }

    fn decrement_counts(&mut self, rules: &[Rule]) {
        for r in rules {
            if let Some(c) = self.counts.get_mut(r) {
                *c -= 1;
            }
        }
    }
}

pub fn induce_pcfg(mut state: ModelState, iterations: usize) -> ModelState {
    for _ in 0..iterations {
        state.resample_parses();
        let _posterior = state.update_rule_posteriors();
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_grammar() -> Grammar {
        Grammar {
            alpha: 0.5,
            rules: vec![
                Rule {
                    lhs: "S".into(),
                    rhs: vec![Symbol::T("a".into())],
                },
                Rule {
                    lhs: "S".into(),
                    rhs: vec![Symbol::T("b".into())],
                },
                Rule {
                    lhs: "S".into(),
                    rhs: vec![Symbol::T("a".into()), Symbol::T("b".into())],
                },
            ],
        }
    }

    #[test]
    fn counts_are_initialized_correctly() {
        let grammar = toy_grammar();
        let parses = vec![
            ParseTree {
                root: "S".into(),
                rules_used: vec![grammar.rules[0].clone()],
                yield_tokens: vec!["a".into()],
            },
            ParseTree {
                root: "S".into(),
                rules_used: vec![grammar.rules[1].clone()],
                yield_tokens: vec!["b".into()],
            },
        ];

        let state = ModelState::new(grammar, parses);
        assert_eq!(state.counts.get(&state.grammar.rules[0]), Some(&1));
        assert_eq!(state.counts.get(&state.grammar.rules[1]), Some(&1));
        assert_eq!(state.counts.get(&state.grammar.rules[2]), None);
    }

    #[test]
    fn posterior_probabilities_sum_to_one_per_lhs() {
        let grammar = toy_grammar();
        let parses = vec![
            ParseTree {
                root: "S".into(),
                rules_used: vec![grammar.rules[0].clone()],
                yield_tokens: vec!["a".into()],
            },
            ParseTree {
                root: "S".into(),
                rules_used: vec![grammar.rules[0].clone()],
                yield_tokens: vec!["a".into()],
            },
        ];

        let state = ModelState::new(grammar, parses);
        let post = state.update_rule_posteriors();
        let sum: f64 = state
            .grammar
            .rules
            .iter()
            .filter(|r| r.lhs == "S")
            .map(|r| post[r])
            .sum();

        assert!((sum - 1.0).abs() < 1e-9);
    }

    #[test]
    fn resampling_preserves_sentence_yield() {
        let grammar = toy_grammar();
        let parses = vec![ParseTree {
            root: "S".into(),
            rules_used: vec![grammar.rules[0].clone()],
            yield_tokens: vec!["a".into()],
        }];

        let mut state = ModelState::new(grammar, parses);
        state.resample_parses();

        assert_eq!(state.parses[0].yield_tokens, vec!["a".to_string()]);
        assert_eq!(state.counts.get(&state.grammar.rules[0]), Some(&1));
    }

    #[test]
    fn induction_runs_multiple_iterations() {
        let grammar = toy_grammar();
        let parses = vec![
            ParseTree {
                root: "S".into(),
                rules_used: vec![grammar.rules[0].clone()],
                yield_tokens: vec!["a".into()],
            },
            ParseTree {
                root: "S".into(),
                rules_used: vec![grammar.rules[1].clone()],
                yield_tokens: vec!["b".into()],
            },
        ];

        let state = ModelState::new(grammar, parses);
        let out = induce_pcfg(state, 3);
        assert_eq!(out.parses.len(), 2);
    }
}