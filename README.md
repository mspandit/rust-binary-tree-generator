# Generating Binary Trees

Code to accompany [this blog post](https://mspandit.github.io/2026/08/24/cfg-parser-combinators.html).

# Execution Expectation
```
$ cargo run
   Compiling rust-binary-tree-generator v0.1.0 (/Users/mspandit/Documents/rust-binary-tree-generator)
warning: function cannot return without recursing
 --> src/binary_string.rs:6:1
  |
6 | pub fn _infinite_reduce1() -> Grammar<char, String> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
7 |     Grammar::Reduce(vec![_infinite_reduce1()])
  |                          ------------------- recursive call site
  |
  = help: a `loop` may express intention better if this is on purpose
  = note: `#[warn(unconditional_recursion)]` on by default

warning: `rust-binary-tree-generator` (bin "rust-binary-tree-generator") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.14s
     Running `target/debug/rust-binary-tree-generator`
42 nonterminal(s)
(a (b (c (d (e f)))))
(a (b (c ((d e) f))))
(a (b ((c d) (e f))))
(a (b ((c (d e)) f)))
(a (b (((c d) e) f)))
(a ((b c) (d (e f))))
(a ((b c) ((d e) f)))
(a ((b (c d)) (e f)))
(a (((b c) d) (e f)))
(a ((b (c (d e))) f))
(a ((b ((c d) e)) f))
(a (((b c) (d e)) f))
(a (((b (c d)) e) f))
(a ((((b c) d) e) f))
((a b) (c (d (e f))))
((a b) (c ((d e) f)))
((a b) ((c d) (e f)))
((a b) ((c (d e)) f))
((a b) (((c d) e) f))
((a (b c)) (d (e f)))
((a (b c)) ((d e) f))
(((a b) c) (d (e f)))
(((a b) c) ((d e) f))
((a (b (c d))) (e f))
((a ((b c) d)) (e f))
(((a b) (c d)) (e f))
(((a (b c)) d) (e f))
((((a b) c) d) (e f))
((a (b (c (d e)))) f)
((a (b ((c d) e))) f)
((a ((b c) (d e))) f)
((a ((b (c d)) e)) f)
((a (((b c) d) e)) f)
(((a b) (c (d e))) f)
(((a b) ((c d) e)) f)
(((a (b c)) (d e)) f)
((((a b) c) (d e)) f)
(((a (b (c d))) e) f)
(((a ((b c) d)) e) f)
((((a b) (c d)) e) f)
((((a (b c)) d) e) f)
(((((a b) c) d) e) f)
1 nonterminal(s)
((the cat) (sat (on (the mat))))
5 nonterminal(s)
(-1 + (2 * 4))
((-1 + 2) * 4)
(-(1 + 2) * 4)
-(1 + (2 * 4))
-((1 + 2) * 4)
```

# Testing Expectation
```
$ cargo test
   Compiling rust-binary-tree-generator v0.1.0 (/Users/mspandit/Documents/rust-binary-tree-generator)
warning: function cannot return without recursing
 --> src/binary_string.rs:6:1
  |
6 | pub fn _infinite_reduce1() -> Grammar<char, String> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
7 |     Grammar::Reduce(vec![_infinite_reduce1()])
  |                          ------------------- recursive call site
  |
  = help: a `loop` may express intention better if this is on purpose
  = note: `#[warn(unconditional_recursion)]` on by default

warning: `rust-binary-tree-generator` (bin "rust-binary-tree-generator" test) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.16s
     Running unittests src/main.rs (target/debug/deps/rust_binary_tree_generator-6f39b8e47bf9cfa5)

running 53 tests
test binary_string::test::test_infinite_shift ... ok
test binary_string::test::test_last ... ok
test expression::test::test_character_or_parse1 ... ok
test binary_string::test::test_binary_from_scratch8 ... ok
test expression::test::test_character_or_parse ... ok
test binary_string::test::test_binary_from_scratch1 ... ok
test expression::test::test_character_parse_non_char ... ok
test expression::test::test_character_parse ... ok
test expression::test::test_character_plus_parse0 ... ok
test binary_string::test::test_binary_from_scratch7 ... ok
test expression::test::test_character_star_parse0 ... ok
test expression::test::test_character_plus_parse1 ... ok
test expression::test::test_digit_or_letter_parse ... ok
test expression::test::test_character_then_parse ... ok
test expression::test::test_digit_parse ... ok
test expression::test::test_digit_parse_non_digit ... ok
test expression::test::test_digit_star_parse ... ok
test expression::test::test_digit_or_letter_star_parse ... ok
test expression::test::test_expression1 ... ok
test expression::test::test_expression2 ... ok
test expression::test::test_expr_parse ... ok
test expression::test::test_expression3 ... ok
test binary_string::test::test_binary_string ... ok
test expression::test::test_expr_parse_fail1 ... ok
test expression::test::test_factor_parse ... ok
test expression::test::test_factor_parse1 ... ok
test expression::test::test_expression4 ... ok
test expression::test::test_integer_parse ... ok
test expression::test::test_item ... ok
test expression::test::test_item_then ... ok
test expression::test::test_item_then_item_then1 ... ok
test expression::test::test_item_then_item_then2 ... ok
test expression::test::test_expr_parse_fail2 ... ok
test expression::test::test_expr_parse_with_parentheses1 ... ok
test expression::test::test_nat_parse ... ok
test expression::test::test_one_character ... ok
test expression::test::test_four_characters ... ok
test expression::test::test_sat ... ok
test expression::test::test_three_characters ... ok
test expression::test::test_two_characters1 ... ok
test expression::test::test_two_characters2 ... ok
test expression::test::test_zero_characters ... ok
test expression::test::test_term_parse ... ok
test grammar::test::test_failure ... ok
test grammar::test::test_or_identity ... ok
test grammar::test::test_simple_grammar3 ... ok
test sentence::test::test_one_word ... ok
test sentence::test::test_six_words ... ok
test sentence::test::test_two_words ... ok
test expression::test::test_expression5 ... ok
test expression::test::test_expression6a ... ok
test expression::test::test_expression6b ... ok
test expression::test::test_expr_parse_with_parentheses2 ... ok

test result: ok. 53 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s
```
