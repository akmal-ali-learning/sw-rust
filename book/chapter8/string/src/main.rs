fn main() {
	let mut s = String::new();
	let data = "Initial contents";

	s = data.to_string();
	let s = "Second contents".to_string();

	// Strings are UTF-8 encoded.
	let hello = String::from("السلام عليكم");
	println!("{}", hello );


	let mut s = String::from("foo");
	s.push_str("bar");

	let mut s1 = String::from("foo");
	let s2 = "bar";
	s1.push_str(s2);
	println!("s2 is {}", s2);
	println!("s1 is {}", s1);


	let mut s = String::from("lo");
	s.push('l');

	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
	println!("s2 is {}", s2);
	println!("s3 is {}", s3);


	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = format!("{}-{}-{}", s1, s2, s3);
	println!("s is {}", s);


	let s1 = String::from("hello");
	//ERROR: cant index String let h = s1[0];


	let hello = "Здравствуйте";

	let s = &hello[0..4];
	println!("s {}", s);


	for c in "नमस्ते".chars() {
	    println!("{}", c);
	}

}
