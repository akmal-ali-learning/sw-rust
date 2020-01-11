 // Chapter 1: Problem 5: 
 //One  Away:  There  are  three  types  of edits  that  can  be  performed  on  strings:  insert  a  character,  remove  a  character, or replace  a  character.  Given  two strings, write  a function to check if   they are one edit  (or zero edits)  away. 


#[cfg(test)]
mod problem_5_test {
	use super::*;
    #[test]
    fn one_away_test() {
    	let x = String::from("tacocat");
    	let a = String::from("tacocab");
    	let b = String::from("tacoca");
    	let c = String::from("tacocats");
    	assert!(is_one_away(&x,&a));
    	assert!(is_one_away(&x,&b));
    	assert!(is_one_away(&x,&c));

    	assert!(!is_one_away(&b,&c));
    	assert!(!is_one_away(&c,&b));
    	assert!(!is_one_away(&a,&c));

    	assert!(is_one_away(&a,&b));
    	assert!(is_one_away(&x,&x));
    	assert!(is_one_away(&a,&a));
    	assert!(is_one_away(&b,&b));
    	assert!(is_one_away(&c,&c));
    }

    #[test]
    fn two_away_test() {
    	let x = String::from("tacocat");
    	let a = String::from("tabocab");
    	let b = String::from("tasoca");
    	let c = String::from("tarocats");
    	assert!(!is_one_away(&x,&a));
    	assert!(!is_one_away(&x,&b));
    	assert!(!is_one_away(&x,&c));

    	assert!(!is_one_away(&b,&c));
    	assert!(!is_one_away(&c,&b));
    	assert!(!is_one_away(&a,&c));

    }
}

pub fn is_one_away(input_a :&String , input_b :&String) -> bool
{
	// Check if strings are one char away from each other.
	if input_a == input_b
	{
		return true
	}

	// If the strings' lengths are different by more than one character
	// they are more than one edit away.
	if (input_a.len() > input_b.len() + 1) || (input_b.len() > input_a.len() + 1) 
	{
		return false
	}

	// Initialise char vectors.
	let mut string_a = Vec::new();
	let mut string_b = Vec::new();

	for ch in input_a.chars()
	{
		string_a.push(ch);
	}

	for ch in input_b.chars()
	{
		string_b.push(ch);
	}

	if  string_a.len() == string_b.len() 
	{
		let mut num_different = 0;
		for i_char in  0 .. string_a.len()
		{
			if  string_a[i_char] != string_b[i_char] 
			{
				num_different += 1;
			}
			if num_different > 1
			{
				return false;
			}
		}
		if num_different == 1
		{
			return true;
		}
	}


	if  string_a.len() > string_b.len() 
	{
		// If string a is 1 char longer than string b, then removing a single character from a can match b.
		return is_one_remove_away(&string_a, &string_b);
	}
	else
	{
		return is_one_remove_away(&string_b, &string_a);
	}
}

fn is_one_remove_away( bigger : &Vec<char> , smaller : &Vec<char>) -> bool {
	assert!( bigger.len() == smaller.len() + 1 );

	for i_char in 0 .. bigger.len()
	{
		let mut test = bigger.clone();
		test.remove(i_char);

		if  &test == smaller 
		{
			return true;
		}
	}
	return false;
}