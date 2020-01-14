use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let teams  = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];
	let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


	// Hash maps and ownership


	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");

	let mut map = HashMap::new();

	map.insert(&field_name,&field_value);

	println!("{} : {}", field_name, field_value );

	// Accessing values in a hash map
	let value = map.get(&field_name);

	let value = match value {
		None => "None",
		Some(i) => i,
	};

	println!("{} : {}", field_name, value);

	let new_colour = String::from("Red");

	// Overwriting a value
	map.insert(&field_name, &new_colour );

	let value = map.get(&field_name);

	let value = match value {
		None => "None",
		Some(i) => i,
	};

	println!("{} : {}", field_name, value);

	let new_field_name = String::from("Old Favorite colour");

	// map.insert(&new_field_name,&new_colour);

	map.entry(&new_field_name).or_insert(&field_value);


	println!("{} : {}", new_field_name, map.entry(&new_field_name).or_insert(&field_value) );
	println!("{:?}", map);

	// Updating a value based on the old value.

	let text = "hello hello hello hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
	    let count = map.entry(word).or_insert(0);
	    *count += 1;
	}
	println!("{:?}", map);


}
