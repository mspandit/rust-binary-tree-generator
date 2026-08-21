mod grammar;
mod sentence;
use crate::sentence::sentence;
mod binary_string;
use crate::binary_string::binary_string;
mod expression;
use crate::expression::expression;

fn main() {
    let character_sequence = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let x = binary_string().parse(& character_sequence);
    println!("{} nonterminal(s)", x.len());
    let _: Vec<Vec<()>> = x.iter()
    .map(|t| t.map(|s| println!("{s}")))
    .collect();
    let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let x = sentence().parse(&word_sequence);
    println!("{} nonterminal(s)", x.len());
    let _: Vec<Vec<()>> = x.iter()
    .map(|t| t.map(|s| println!("{s:?}")))
    .collect();
    let char_sequence = vec!['-', '1', '+', '2', '*', '4'];
    let x = expression().parse(& char_sequence);
    println!("{} nonterminal(s)", x.len());
    let _: Vec<Vec<()>> = x.iter()
    .map(|t| t.map(|s| println!("{s:?}")))
    .collect();
}
