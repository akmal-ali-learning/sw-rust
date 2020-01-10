 // Chapter 1: Problem 4: Palindrome Permutation: Given  a  string, write a function to check if it is a  permutation of a palin-drome.  A palindrome is a word  or phrase that is the same forwards and  backwards.  A permutation is  a  rearrangement of letters. The palindrome does not  need  to  be limited  to just dictionary words. 

use std::collections::HashMap; // HashMap has been imported.

#[cfg(test)]
mod problem_4_test {
	use super::*;
    #[test]
    fn is_palindrome_permuted_test() {
    	let x = String::from("tacocat");
    	assert!(is_palindrome_permuted(&x));

    	let x = String::from("123454312");
    	assert!(is_palindrome_permuted(&x));

    	let x = String::from("12345674312");
    	assert!(!is_palindrome_permuted(&x));
    }

    #[test]
    fn is_palindrome_permuted_test2() {
    	let x = String::from("");
    	assert!(!is_palindrome_permuted(&x));

    	let x = String::from("*");
    	assert!(is_palindrome_permuted(&x));

    	let x = String::from("#-");
    	assert!(!is_palindrome_permuted(&x));
    }
}

pub fn is_palindrome_permuted( input : &String ) -> bool {
	if input.len() == 0
	{
		return false
	}

	let mut char_count : HashMap< char, u32 > = HashMap::new();

	// Step 1 : Get the frequency of each character in the string.
	for char_i in input.chars()
	{
		let counter = char_count.entry(char_i).or_insert(0);
		*counter += 1;
	} 

	// Step 2 : Is it a permutation of a palindrome?
	let mut num_odd = 0;
	for (_key,value) in char_count
	{
		if value % 2 == 1
		{
			num_odd += 1;
			if num_odd > 1
			{
				return false;
			}
		}

	}
	return true;
}


