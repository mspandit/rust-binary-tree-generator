use crate::grammar::{Grammar, recursive};
mod grammar;

use crate::sentence::{noun, noun_phrase, sentence};
mod sentence;

mod binary_string;
mod expression;

fn main() {
    let countdown = recursive(&|factorial: &Grammar<i32, ()>, x: &i32| {
        if *x <= 0 {
            println!("Blastoff!");
        } else {
            println!("Counting down: {}", *x);
            factorial.apply(&(x - 1));
        }
        vec![]
    });
    countdown.apply(&3);

    let x = noun().parse(& vec!["cat".to_string()]);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    println!("---------------------");
    let x = noun().parse(& vec!["mat".to_string()]);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    println!("---------------------");
    let word_sequence = vec!["the", "cat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let x = noun_phrase().parse(&word_sequence);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
    println!("---------------------");
    let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let x = sentence().parse(&word_sequence);
    println!("{} trees", x.len());
    for t in x {
        println!("{:?}", t);
    }
}

#[cfg(test)]
mod test {

use super::*;
}