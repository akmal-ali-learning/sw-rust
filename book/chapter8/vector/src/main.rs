fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

	for i in &v {
	    println!("{}", i);
	}

	// let does_not_exist = &v[100];
	let does_not_exist = v.get(100);



	let mut v = vec![100, 32, 57];

	for i in &mut v {
	    *i += 50;
	}

	let first = &v[0];

	println!("first {}", first );


	// Using an enum to store multiple types.
	enum SpreadsheetCell {
	    Int(i32),
	    Float(f64),
	    Text(String),
	}

	let mut row = vec![
	    SpreadsheetCell::Int(3),
	    SpreadsheetCell::Text(String::from("blue")),
	    SpreadsheetCell::Float(10.12),
	    SpreadsheetCell::Float(20.12),
	    SpreadsheetCell::Float(30.12),
	    SpreadsheetCell::Float(40.12),
	];

	while 0 != row.len() {
		row.pop();
	}


}
