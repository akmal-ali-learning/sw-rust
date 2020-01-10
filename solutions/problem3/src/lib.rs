// Chapter 1 [Arrays and Strings]

// Problem 3. Check Permutation: Given two strings, write a method to decide if one is a permutation of the other.

// Write a  method  to replace all  spaces in  a  string with '%20'. You  may assume that the string has  sufficient  space at the end  to  hold the additional  characters, and  that you  are given  the "true" length  of the  string.  (Note:  If    implementing  in  Java,  please  use  a  character  array  so  that  you  can  perform this operation  in  place.)

#[cfg(test)]
mod problem_3_test {
    use super::*;
    #[test]
    fn test_1() {
        let mut x = String::from("Mr 3ohn Smith    ");
        let num_characters = 13;
        urlify(&mut x, num_characters);
        assert_eq!(x, "Mr%203ohn%20Smith");
    }

        #[test]
    fn test_2() {
        let mut x = String::from("Mr 3ohn Smith ABCD     ");
        let num_characters = 18;
        urlify(&mut x, num_characters);
        assert_eq!(x, "Mr%203ohn%20Smith%20ABCD");
    }

    
}

fn urlify(input: &mut String, true_chars: usize ) {
    let mut a = String::new();

    for i_char in 0 .. true_chars {
    	let c = input.chars().nth(i_char).unwrap();
    	if c == ' ' 
    	{
            a.push('%');
            a.push('2');
            a.push('0');
        } else {
            a.push(c);
        }
    }
    *input = a.clone()
}
