// Chapter 1: Problem 6:
// String  Compression: Implement  a  method  to  perform  basic string compression  using the counts of    repeated  characters.  For  example,  the  string  aabcccccaaa  would  become  a2b1c5a3,  If the "compressed" string would  not  become  smaller than the original  string, your method  should  return  the original string. You  can assume the string has only uppercase and lowercase letters (a - z).

#[cfg(test)]
mod problem_6_test {
    use super::*;

    #[test]
    fn compress_string_test() {
        let x = String::from("aabcccccaaa");
        assert!(compress_string(&x) == "a2b1c5a3");

        let x = String::from("x");
        assert!(compress_string(&x) == "x");

        let x = String::from("xxxxxaaaaabbbbbcccccdddddeeeeefffffggggghhhhhiiiii");
        assert!(compress_string(&x) == "x5a5b5c5d5e5f5g5h5i5");

        let x = String::from("abcdeee");
        assert!(compress_string(&x) == "abcdeee");

        let x = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert!(compress_string(&x) == "a100");

        let x = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaadaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

        assert!(compress_string(&x) == "a52d1a47");
    }
}

pub fn compress_string(input: &String) -> String {
	let mut string_b = String::new();

	let mut prev_char = '#';

	let mut cur_char_count = 0;

	for chars in input.chars(){

		if chars == prev_char
		{
			cur_char_count += 1;
		}
		else {
			if  cur_char_count > 0 
			{
				let x = cur_char_count.to_string();
				for count_chars in x.chars()
				{
					string_b.push(count_chars);
				}
			}
			cur_char_count = 1;
			string_b.push(chars);

		}
		prev_char = chars
	}
	if  cur_char_count > 0 
	{
		let x = cur_char_count.to_string();
		for count_chars in x.chars()
		{
			string_b.push(count_chars);
		}
	}

	if  string_b.len() >= input.len() 
	{
		return input.clone()
	}
	else{
		println!("string_b = {} ", string_b);
		return string_b;
	}
}
