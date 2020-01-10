// Chapter 1 [Arrays and Strings]

// Problem 1. Is Unique? Implement an algorithm to detemine if a string has all unique characters?
// What if you cannot use addition characters
use std::collections::HashMap;

pub fn is_string_unique( input : &String) -> bool {

	if input.len() > 0
	{
		for x in 0 .. input.len()-1
		{
			for y in x + 1 .. input.len()-1
			{
				if input.as_bytes()[y] == input.as_bytes()[x]
				{
					println!("{:?}", input.as_bytes()[x] );
					return false
				}
			}
			// println!("{:?}", input.as_bytes()[x] );
		}
	}

	return true
}


pub fn is_string_unique_hashmap( input : &String) -> bool {
	let mut char_map = HashMap::new();
	for chars in input.chars()
	{
		let x = char_map.entry(chars).or_insert(0);
		*x = *x + 1;
		// char_map.insert(chars,"0")
		if *x > 1
		{
			return false;
		}
	}
	return true
}




#[cfg(test)]
mod problem_1_test {
	use super::*;
    #[test]
    fn unique_string_test() {
        let x = String::from("abcdefgh");

        assert!(is_string_unique(&x));
        assert!(is_string_unique_hashmap(&x));
    }

    #[test]
    fn not_unique_string_test() {
        let x = String::from("abcdafgh");

        assert!(!is_string_unique(&x));
        assert!(!is_string_unique_hashmap(&x));
    }

    #[test]
    fn empty_string() {
        let x = String::from("");

        assert!(is_string_unique(&x));
        assert!(is_string_unique_hashmap(&x));
    }



}
