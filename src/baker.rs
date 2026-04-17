use std::collections::HashMap;

type NonTerminal = String;
type Terminal = String;

// Rule representation
#[derive(Clone)]
enum RuleRHS {
    Binary(NonTerminal, NonTerminal),
    Terminal(Terminal),
}

#[derive(Clone)]
struct Rule {
    lhs: NonTerminal,
    rhs: RuleRHS,
    prob: f64,
}

// Grammar
struct Grammar {
    rules: Vec<Rule>,
    start: NonTerminal,
}

// Inside table: β[A][i][j]
type InsideTable = HashMap<(NonTerminal, usize, usize), f64>;

// Outside table: α[A][i][j]
type OutsideTable = HashMap<(NonTerminal, usize, usize), f64>;

/// Compute inside probabilities
fn inside_probabilities(
    sentence: &[String],
    grammar: &Grammar,
) -> InsideTable {
    let n = sentence.len();
    let mut beta: InsideTable = HashMap::new();

    // Base case
    for i in 0..n {
        for rule in &grammar.rules {
            if let RuleRHS::Terminal(ref t) = rule.rhs {
                if t == &sentence[i] {
                    *beta.entry((rule.lhs.clone(), i, i)).or_insert(0.0) += rule.prob;
                }
            }
        }
    }

    // Recursive case
    for span_len in 2..=n {
        for i in 0..=n - span_len {
            let j = i + span_len - 1;

            for k in i..j {
                for rule in &grammar.rules {
                    if let RuleRHS::Binary(ref b, ref c) = rule.rhs {
                        let left = *beta.get(&(b.clone(), i, k)).unwrap_or(&0.0);
                        let right = *beta.get(&(c.clone(), k + 1, j)).unwrap_or(&0.0);

                        if left > 0.0 && right > 0.0 {
                            *beta.entry((rule.lhs.clone(), i, j)).or_insert(0.0) +=
                                rule.prob * left * right;
                        }
                    }
                }
            }
        }
    }

    beta
}

/// Compute outside probabilities
fn outside_probabilities(
    sentence: &[String],
    grammar: &Grammar,
    beta: &InsideTable,
) -> OutsideTable {
    let n = sentence.len();
    let mut alpha: OutsideTable = HashMap::new();

    // Initialize
    alpha.insert((grammar.start.clone(), 0, n - 1), 1.0);

    // Iterate spans
    for span_len in (1..=n).rev() {
        for i in 0..=n - span_len {
            let j = i + span_len - 1;

            for rule in &grammar.rules {
                if let RuleRHS::Binary(ref b, ref c) = rule.rhs {
                    for k in i..j {
                        let parent = *alpha.get(&(rule.lhs.clone(), i, j)).unwrap_or(&0.0);
                        if parent == 0.0 {
                            continue;
                        }

                        let left = *beta.get(&(b.clone(), i, k)).unwrap_or(&0.0);
                        let right = *beta.get(&(c.clone(), k + 1, j)).unwrap_or(&0.0);

                        // Update left child
                        if right > 0.0 {
                            *alpha.entry((b.clone(), i, k)).or_insert(0.0) +=
                                parent * rule.prob * right;
                        }

                        // Update right child
                        if left > 0.0 {
                            *alpha.entry((c.clone(), k + 1, j)).or_insert(0.0) +=
                                parent * rule.prob * left;
                        }
                    }
                }
            }
        }
    }

    alpha
}

/// One EM iteration
fn em_step(
    corpus: &[Vec<String>],
    grammar: &mut Grammar,
) {
    let mut counts: HashMap<usize, f64> = HashMap::new();

    for sentence in corpus {
        let beta = inside_probabilities(sentence, grammar);
        let alpha = outside_probabilities(sentence, grammar, &beta);

        let n = sentence.len();
        let z = *beta.get(&(grammar.start.clone(), 0, n - 1)).unwrap_or(&1e-10);

        for (r_idx, rule) in grammar.rules.iter().enumerate() {
            let mut count = 0.0;

            for i in 0..n {
                for j in i..n {
                    if let RuleRHS::Binary(ref b, ref c) = rule.rhs {
                        for k in i..j {
                            let a_val = *alpha.get(&(rule.lhs.clone(), i, j)).unwrap_or(&0.0);
                            let b_val = *beta.get(&(b.clone(), i, k)).unwrap_or(&0.0);
                            let c_val = *beta.get(&(c.clone(), k + 1, j)).unwrap_or(&0.0);

                            count += a_val * rule.prob * b_val * c_val / z;
                        }
                    } else if let RuleRHS::Terminal(ref t) = rule.rhs {
                        if i == j && &sentence[i] == t {
                            let a_val = *alpha.get(&(rule.lhs.clone(), i, i)).unwrap_or(&0.0);
                            count += a_val * rule.prob / z;
                        }
                    }
                }
            }

            *counts.entry(r_idx).or_insert(0.0) += count;
        }
    }

    // Normalize counts
    let mut lhs_totals: HashMap<NonTerminal, f64> = HashMap::new();

    for (r_idx, rule) in grammar.rules.iter().enumerate() {
        let c = counts.get(&r_idx).unwrap_or(&0.0);
        *lhs_totals.entry(rule.lhs.clone()).or_insert(0.0) += c;
    }

    for (r_idx, rule) in grammar.rules.iter_mut().enumerate() {
        let c = counts.get(&r_idx).unwrap_or(&0.0);
        let total = lhs_totals.get(&rule.lhs).unwrap_or(&1e-10);

        rule.prob = c / total;
    }
}

/// Full training loop
fn train_pcfg(
    corpus: &[Vec<String>],
    grammar: &mut Grammar,
    iterations: usize,
) {
    for _ in 0..iterations {
        em_step(corpus, grammar);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_grammar() -> Grammar {
        Grammar {
            start: "S".to_string(),
            rules: vec![
                Rule {
                    lhs: "S".to_string(),
                    rhs: RuleRHS::Binary("NP".to_string(), "VP".to_string()),
                    prob: 1.0,
                },
                Rule {
                    lhs: "NP".to_string(),
                    rhs: RuleRHS::Terminal("dogs".to_string()),
                    prob: 0.5,
                },
                Rule {
                    lhs: "NP".to_string(),
                    rhs: RuleRHS::Terminal("cats".to_string()),
                    prob: 0.5,
                },
                Rule {
                    lhs: "VP".to_string(),
                    rhs: RuleRHS::Terminal("run".to_string()),
                    prob: 1.0,
                },
            ],
        }
    }

    #[test]
    fn test_inside_base_case() {
        let grammar = simple_grammar();
        let sentence = vec!["dogs".to_string()];

        let beta = inside_probabilities(&sentence, &grammar);

        let val = beta.get(&("NP".to_string(), 0, 0)).unwrap();
        assert!((*val - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_inside_full_sentence() {
        let grammar = simple_grammar();
        let sentence = vec!["dogs".to_string(), "run".to_string()];

        let beta = inside_probabilities(&sentence, &grammar);

        let val = beta.get(&("S".to_string(), 0, 1)).unwrap();
        assert!((*val - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_outside_root_initialization() {
        let grammar = simple_grammar();
        let sentence = vec!["dogs".to_string(), "run".to_string()];

        let beta = inside_probabilities(&sentence, &grammar);
        let alpha = outside_probabilities(&sentence, &grammar, &beta);

        let root = alpha.get(&("S".to_string(), 0, 1)).unwrap();
        assert!((*root - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_outside_propagation() {
        let grammar = simple_grammar();
        let sentence = vec!["dogs".to_string(), "run".to_string()];

        let beta = inside_probabilities(&sentence, &grammar);
        let alpha = outside_probabilities(&sentence, &grammar, &beta);

        let np_val = alpha.get(&("NP".to_string(), 0, 0)).unwrap();
        assert!(*np_val > 0.0);

        let vp_val = alpha.get(&("VP".to_string(), 1, 1)).unwrap();
        assert!(*vp_val > 0.0);
    }

    #[test]
    fn test_em_step_preserves_valid_probabilities() {
        let mut grammar = simple_grammar();
        let corpus = vec![
            vec!["dogs".to_string(), "run".to_string()],
            vec!["cats".to_string(), "run".to_string()],
        ];

        em_step(&corpus, &mut grammar);

        // Check probabilities are valid
        for rule in &grammar.rules {
            assert!(rule.prob >= 0.0 && rule.prob <= 1.0);
        }

        // Check normalization
        let mut lhs_totals: HashMap<String, f64> = HashMap::new();
        for rule in &grammar.rules {
            *lhs_totals.entry(rule.lhs.clone()).or_insert(0.0) += rule.prob;
        }

        for (_, total) in lhs_totals {
            assert!((total - 1.0).abs() < 1e-6);
        }
    }
    fn ambiguous_grammar() -> Grammar {
        Grammar {
            start: "S".to_string(),
            rules: vec![
                Rule {
                    lhs: "S".to_string(),
                    rhs: RuleRHS::Binary("NP".to_string(), "VP".to_string()),
                    prob: 0.5,
                },
                Rule {
                    lhs: "S".to_string(),
                    rhs: RuleRHS::Binary("VP".to_string(), "NP".to_string()),
                    prob: 0.5,
                },
                Rule {
                    lhs: "NP".to_string(),
                    rhs: RuleRHS::Terminal("dogs".to_string()),
                    prob: 0.5,
                },
                Rule {
                    lhs: "NP".to_string(),
                    rhs: RuleRHS::Terminal("cats".to_string()),
                    prob: 0.5,
                },
                Rule {
                    lhs: "VP".to_string(),
                    rhs: RuleRHS::Terminal("run".to_string()),
                    prob: 1.0,
                },
            ],
        }
    }

    #[test]
    fn test_em_step_updates_probabilities_with_ambiguity() {
        let mut grammar = ambiguous_grammar();
        let corpus = vec![
            vec!["dogs".to_string(), "run".to_string()],
        ];

        let before: Vec<f64> = grammar.rules.iter().map(|r| r.prob).collect();

        em_step(&corpus, &mut grammar);

        let after: Vec<f64> = grammar.rules.iter().map(|r| r.prob).collect();

        let changed = before.iter().zip(after.iter()).any(|(b, a)| {
            (b - a).abs() > 1e-6
        });

        assert!(changed, "Expected probabilities to change under ambiguity");
    }

    #[test]
    fn test_probability_normalization() {
        let mut grammar = simple_grammar();
        let corpus = vec![
            vec!["dogs".to_string(), "run".to_string()],
            vec!["cats".to_string(), "run".to_string()],
        ];

        em_step(&corpus, &mut grammar);

        let mut lhs_totals: HashMap<String, f64> = HashMap::new();

        for rule in &grammar.rules {
            *lhs_totals.entry(rule.lhs.clone()).or_insert(0.0) += rule.prob;
        }

        for (_, total) in lhs_totals {
            assert!((total - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_training_runs_multiple_iterations() {
        let mut grammar = simple_grammar();
        let corpus = vec![
            vec!["dogs".to_string(), "run".to_string()],
            vec!["cats".to_string(), "run".to_string()],
        ];

        train_pcfg(&corpus, &mut grammar, 5);

        // Check probabilities still valid
        for rule in &grammar.rules {
            assert!(rule.prob >= 0.0 && rule.prob <= 1.0);
        }
    }

    #[test]
    fn test_sentence_probability_nonzero() {
        let grammar = simple_grammar();
        let sentence = vec!["dogs".to_string(), "run".to_string()];

        let beta = inside_probabilities(&sentence, &grammar);

        let prob = beta.get(&("S".to_string(), 0, 1)).unwrap();
        assert!(*prob > 0.0);
    }

    #[test]
    fn test_unseen_word_gives_zero_probability() {
        let grammar = simple_grammar();
        let sentence = vec!["birds".to_string()];

        let beta = inside_probabilities(&sentence, &grammar);

        assert!(beta.is_empty() || beta.values().all(|&v| v == 0.0));
    }
}
