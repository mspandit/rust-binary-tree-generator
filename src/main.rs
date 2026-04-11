use std::fmt::Display;

trait Token: Clone + Display + Default{}

#[derive(Debug, Clone)]
enum BinaryTree<T: Token> {
    Terminal(T),
    Nonterminal(Box<BinaryTree<T>>, Box<BinaryTree<T>>),
}

impl<T: Token> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryTree::Terminal(c) => write!(f, "{c}"),
            BinaryTree::Nonterminal(left, right) =>
                write!(f, "({left} {right})"),
        }
    }
}

struct PoppingIterator<T: Token>(Stack<T>);

impl<T: Token> Iterator for PoppingIterator<T> {
    type Item = (BinaryTree<T>, Stack<T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.0.pop().and_then(|tree| Some((
            tree, // The tree that was just popped off the stack
            self.0.clone(), // Copy of the stack after pop
        )))
    }
}

#[derive(Clone, Default)]
struct Stack<T: Token>(Vec<BinaryTree<T>>);

impl<T: Token> Stack<T> {
    fn popping_iter(self: Self) -> PoppingIterator<T> {
        PoppingIterator(self)
    }
}

fn reduce0<T: Token>(new: (Vec<Stack<T>>, BinaryTree<T>), popped: (BinaryTree<T>, Stack<T>))
-> (Vec<Stack<T>>, BinaryTree<T>) {
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

fn reduce<T: Token>(stack: Stack<T>, current: T) -> Vec<Stack<T>> {
    stack.popping_iter().fold(
        (Vec::default(), BinaryTree::Terminal(current)),
        reduce0
    ).0
}

fn shift_reduce_fn<T: Token>(current: T) -> impl Fn(Vec<Stack<T>>, Stack<T>) -> Vec<Stack<T>> {
    move |mut new_stacks, mut stack| {
        // reduce may produce multiple stacks
        new_stacks.append(& mut reduce(stack.clone(), current.clone()));
        // shift
        stack.0.push(BinaryTree::Terminal(current.clone()));
        new_stacks.push(stack); // transfer ownership
        new_stacks
    }
}


fn tops<T: Token>(stacks: Vec<Stack<T>>) -> Vec<BinaryTree<T>> {
    stacks.iter()
        .map(|stack| stack.0[0].clone())
        .collect()
}

fn filter_stacks<T: Token>(stacks: Vec<Stack<T>>) -> Vec<Stack<T>> {
    stacks.into_iter()
        .filter(|stack| 1 == stack.0.len())
        .collect()
}

impl Token for char {}
impl Token for &str {}

fn generate_stacks<T: Token>(input_sequence: impl Iterator<Item = T>)
-> Vec<Stack<T>> {
    input_sequence.fold(
        vec![Stack::default()],
        |stacks, input| {
            stacks.into_iter().fold(
                Vec::default(),
                shift_reduce_fn(input)
            )
        }
    )
}

fn generate<T: Token>(input_sequence: impl Iterator<Item = T>)
-> Vec<BinaryTree<T>> {
    tops(filter_stacks(generate_stacks(input_sequence)))
}

fn main() {
    let x = generate("abcde".chars());
    println!("{} trees", x.len());
    for t in x {
        println!("{}", t);
    }
    let word_sequence = vec!["the", "cat", "sat", "on", "the", "mat"];
    let x = generate(word_sequence.into_iter());
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
        let x = generate_stacks("".chars());
        assert_eq!(1, x.len());
        assert_eq!(0, x[0].0.len());
        let x = generate("".chars());
        assert_eq!(0, x.len());
    }

    #[test]
    fn test_reduce_second_character() {
        let stack = Stack(vec![BinaryTree::Terminal('a')]);
        let x = reduce(stack, 'c');
        assert_eq!(1, x.len())
    }

    #[test]
    fn test_reduce_third_character() {
        let stack = Stack(vec![
            BinaryTree::Terminal('b'),
            BinaryTree::Terminal('a'),
        ]);
        let x = reduce(stack, 'c');
        assert_eq!(2, x.len())
    }

    #[test]
    fn test_one_or_two_characters() {
        let x = generate_stacks("a".chars());
        assert_eq!(1, x.len());
        assert_eq!(1, x[0].0.len());
        let x = generate("a".chars());
        assert_eq!(1, x.len());
        let x = generate_stacks("ab".chars());
        assert_eq!(2, x.len());
        let x = generate("ab".chars());
        assert_eq!(1, x.len());
    }

    #[test]
    fn test_three_characters() {
        let x = generate("abc".chars());
        assert_eq!(2, x.len(), "{:?}", x);
    }

    #[test]
    fn test_four_characters() {
        let x = generate("abcd".chars());
        assert_eq!(5, x.len());
    }
}