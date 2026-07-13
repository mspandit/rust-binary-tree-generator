use crate::grammar::{Grammar, recursive};


pub fn binary_string() -> Grammar<char, String> {
    recursive(&|binary_string: &Grammar<char, String>, x: &char| {
        let binary_string1 = binary_string.clone();
        binary_string1.clone().then(move |l|{
            let binary_string2 = binary_string1.clone();
            binary_string2.then(move |r|
                format!("({l} {r})").to_string().into()
            )
        })
        .or(x.to_string().into()).apply(x)
    })
}

#[cfg(test)]
mod test {

use super::*;

    // #[test]
    // fn test_binary1() {
    //     let x = binary_string().parse(&"a".chars().collect::<Vec<char>>());
    //     assert_eq!(2, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary2() {
    //     let input = "ab".chars().collect();
    //     let x = binary_string().parse(&input);
    //     assert_eq!(2, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary3() {
    //     let input = "abc".chars().collect();
    //     let x = binary_string().parse(&input);
    //     assert_eq!(2, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary4() {
    //     let input = "abcd".chars().collect();
    //     let x = binary_string().parse(&input);
    //     assert_eq!(5, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary5() {
    //     let input = "abcde".chars().collect();
    //     let x = binary_string().parse(&input);
    //     assert_eq!(14, x.len(), "{x:?}");
    // }
    // #[test]
    // fn test_binary6() {
    //     let input = "abcdef".chars().collect();
    //     let x = binary_string().parse(&input);
    //     assert_eq!(42, x.len(), "{x:?}");
    // }
}