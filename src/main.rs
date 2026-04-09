use std::fmt::Display;

#[derive(Debug, Clone)]
enum BinaryTree {
    Terminal(char),
    Nonterminal(Box<BinaryTree>, Box<BinaryTree>),
}

impl Display for BinaryTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryTree::Terminal(c) => write!(f, "{c}"),
            BinaryTree::Nonterminal(left, right) =>
                write!(f, "({left} {right})"),
        }
    }
}

struct PoppingIterator(Stack);

impl Iterator for PoppingIterator {
    type Item = (BinaryTree, Stack);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.0.pop().and_then(|tree| Some((
            tree, // The tree that was just popped off the stack
            self.0.clone(), // Copy of the stack after pop
        )))
    }
}

#[derive(Clone, Default)]
struct Stack(Vec<BinaryTree>);

impl Stack {
    fn popping_iter(self: Self) -> PoppingIterator {
        PoppingIterator(self)
    }
}

fn eager0(new: (Vec<Stack>, BinaryTree), popped: (BinaryTree, Stack))
-> (Vec<Stack>, BinaryTree) {
    let mut new_stacks = new.0;
    let new_tree = new.1;
    let tree = popped.0;
    let stack = popped.1;
    let new_nonterminal = BinaryTree::Nonterminal(
        Box::new(tree),
        Box::new(new_tree),
    );
    let mut new_stack = stack;
    new_stack.0.push(new_nonterminal.clone());
    new_stacks.push(new_stack);
    (new_stacks, new_nonterminal)
}

fn eager(stack: Stack, current: char) -> Vec<Stack> {
    stack.popping_iter().fold(
        (Vec::default(), BinaryTree::Terminal(current)),
        eager0
    ).0
}

fn lazy_eager_fn(current: char) -> impl Fn(Vec<Stack>, Stack)
-> Vec<Stack> {
    move |mut new_stacks, mut stack| {
        // eager may produce multiple stacks
        new_stacks.append(& mut eager(stack.clone(), current));
        // lazy
        stack.0.push(BinaryTree::Terminal(current));
        new_stacks.push(stack); // transfer ownership
        new_stacks
    }
}

fn generate_stacks(input_sequence: & str) -> Vec<Stack> {
    input_sequence.chars().fold(
        vec![Stack::default()],
        |stacks, input| {
            stacks.into_iter().fold(
                Vec::default(),
                lazy_eager_fn(input)
            )
        }
    )
}

fn tops(stacks: Vec<Stack>) -> Vec<BinaryTree> {
    stacks.iter()
        .map(|stack| stack.0[0].clone())
        .collect()
}

fn filter_stacks(stacks: Vec<Stack>) -> Vec<Stack> {
    stacks.into_iter()
        .filter(|stack| 1 == stack.0.len())
        .collect()
}

fn generate(input_sequence: &str) -> Vec<BinaryTree> {
    tops(filter_stacks(generate_stacks(input_sequence)))
}

fn main() {
    let x = generate("abcde");
    println!("{} trees", x.len());
    for t in x {
        println!("{}", t);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zero_characters() {
        let x = generate_stacks("");
        assert_eq!(1, x.len());
        assert_eq!(0, x[0].0.len());
        let x = generate("");
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_eager_second_character() {
        let stack = Stack(vec![BinaryTree::Terminal('a')]);
        let x = eager(stack, 'c');
        assert_eq!(1, x.len())
    }

    #[test]
    fn test_eager_third_character() {
        let stack = Stack(vec![
            BinaryTree::Terminal('b'),
            BinaryTree::Terminal('a'),
        ]);
        let x = eager(stack, 'c');
        assert_eq!(2, x.len())
    }

    #[test]
    fn test_one_or_two_characters() {
        let x = generate_stacks("a");
        assert_eq!(1, x.len());
        assert_eq!(1, x[0].0.len());
        let x = generate("a");
        assert_eq!(1, x.len());
        let x = generate_stacks("ab");
        assert_eq!(2, x.len());
        let x = generate("ab");
        assert_eq!(1, x.len());
    }

    #[test]
    fn test_three_characters() {
        let x = generate("abc");
        assert_eq!(2, x.len(), "{:?}", x);
    }

    #[test]
    fn test_four_characters() {
        let x = generate("abcd");
        assert_eq!(5, x.len());
    }
}