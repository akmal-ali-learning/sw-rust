// Chapter 1 [Arrays and Strings]

// Problem 2. Check Permutation: Given two strings, write a method to decide if one is a permutation of the other.
use std::collections::HashMap;

pub fn is_string_permutation(input_a: &String, input_b: &String) -> bool {
    let mut char_map = HashMap::new();

    for char_a in input_a.chars() {
        let x = char_map.entry(char_a).or_insert(0);
        *x = *x + 1;
    }

    for char_b in input_b.chars() {
        let x = char_map.entry(char_b).or_insert(0);
        *x = *x - 1;
    }

    for (_key, value) in char_map {
        if value != 0 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod problem_2_test {
    use super::*;
    #[test]
    fn permut_string_test1() {
        let string_a = String::from("abcdefgh");
        let string_b = String::from("bcdefgha");
        assert!(is_string_permutation(&string_a, &string_b));
    }

    #[test]
    fn permut_string_test2() {
        let string_a = String::from("abcdefgh");
        let string_b = String::from("acdefgha");
        assert!(!is_string_permutation(&string_a, &string_b));
    }

    #[test]
    fn permut_string_test3() {
        let string_a = String::from("abcddfgh");
        let string_b = String::from("bcddfgha");
        assert!(is_string_permutation(&string_a, &string_b));
    }

    #[test]
    fn permut_string_test4() {
        let string_a = String::from("abcdefgh");
        let string_b = String::from("abcdefg");
        assert!(!is_string_permutation(&string_a, &string_b));
    }

    #[test]
    fn permut_string_test5() {
        let string_a = String::from("abcdeash");
        let string_b = String::from("abcdasfg");
        assert!(!is_string_permutation(&string_a, &string_b));
    }

    #[test]
    fn permut_string_test6() {
        let string_a = String::from("abcdefgh");
        let string_b = String::from("acbdfehg");
        assert!(is_string_permutation(&string_a, &string_b));
    }
}
