use crate::{Token, binary_tree::BinaryTree, grammar::Grammar, stack::Stack};

#[derive(Debug)]
pub struct GeneratorState<T: Token>(Vec<Stack<T>>);

impl<T: Token> Default for GeneratorState<T> {
    fn default() -> Self {
        Self(vec![Stack::default()])
    }
}

fn process_fn<T: Token>(token: T, grammar: &Grammar<T>)
-> impl Fn(Vec<Stack<T>>, Stack<T>) -> Vec<Stack<T>> {
    move |new_stacks, current_stack| {
        grammar.lookup_terminals(& token).map_or(
            new_stacks.clone(),
            |t_labels| t_labels.iter().fold(
                new_stacks,
                |mut new_stacks, terminal_label| {
                    new_stacks.append(&mut current_stack.clone()
                        .shift_reduce(
                            BinaryTree::Terminal {
                                label: terminal_label.clone(),
                                token: token.clone()
                            },
                            grammar
                        )
                    );
                    new_stacks
                }
            )
        )
    }
}

impl<T: Token> GeneratorState<T> {
    #[cfg(test)]
    pub fn len(self: & Self) -> usize {
        self.0.len()
    }

    pub fn process(self: Self, token: T, grammar: &Grammar<T>) -> Self {
        let stack_fn = process_fn(token, grammar);
        Self(
            self.0.into_iter().fold(
                Vec::default(),
                stack_fn
            )
        )
    }

    pub fn tops(self: Self) -> Vec<BinaryTree<T>> {
        self.0.into_iter().map(|stack| stack.top()).collect()
    }

    pub fn filter_stacks(self: Self) -> Self {
        Self(
            self.0.into_iter()
                .filter(|stack| 1 == stack.len())
                .collect()
        )
    }
}
