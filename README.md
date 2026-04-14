# Generating Binary Trees

Code to accompany [this blog post](https://mspandit.github.io/2026/04/13/more-functional-generation.html).

# Execution Expectation
```
$ cargo run
   Compiling rust-binary-tree-generator v0.1.0 (/Users/mspandit/Documents/rust-binary-tree-generator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running `target/debug/rust-binary-tree-generator`
5 trees
(- ((1 +) ((2 *) 4)))
(- ((((1 +) 2) *) 4))
(((- ((1 +) 2)) *) 4)
(((- 1) +) ((2 *) 4))
(((((- 1) +) 2) *) 4)
1 trees
((the cat) (sat (on (the mat))))
```

# Testing Expectation
```
$ cargo test
   Compiling rust-binary-tree-generator v0.1.0 (/Users/mspandit/Documents/rust-binary-tree-generator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running unittests src/main.rs (target/debug/deps/rust_binary_tree_generator-34b116d9e102df5a)

running 11 tests
test stack::test::test_reduce_third_character ... ok
test stack::test::test_reduce_second_character ... ok
test test::test_zero_characters ... ok
test test::test_one_character ... ok
test test::test_one_word ... ok
test test::test_two_words ... ok
test test::test_three_characters ... ok
test test::test_two_characters ... ok
test test::test_six_words ... ok
test test::test_zero_words ... ok
test test::test_four_characters ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
