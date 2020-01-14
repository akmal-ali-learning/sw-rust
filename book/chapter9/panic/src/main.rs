use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {

	if(false){
	    let f=  File::open("hello.txt");

	    let f = match f {
	        Ok(file) => file,
	        Err(error) => match error.kind() {
	        	ErrorKind::NotFound => match File::create("hello.txt") {
	        		Ok(fc) => fc,
	        		Err(e) => panic!("Problem creating the file: {:?} ", e),
	        	},
	        	other_error => panic!("Problem opening the file: {:?}", other_error),
	        },
	    };

	    // @todo in the future learn how to do this.
	    let f = File::open("hello.txt").unwrap_or_else(|error| {
	        if error.kind() == ErrorKind::NotFound {
	            File::create("hello.txt").unwrap_or_else(|error| {
	                panic!("Problem creating the file: {:?}", error);
	            })
	        } else {
	            panic!("Problem opening the file: {:?}", error);
	        }
	    });
	}
    // Simplified. Unwrap calls panic and prints the error message.
    // let f = File::opena("hello.txt").unwrap();

    // Expect allows you to print your own custom error message.
 
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let f = read_username_from_file().expect("Cant read username from file.");

}

// Propogating errors.
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }


// @todo
// Propogating Errors using the ? error.
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }


//Even simpler example
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }


fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The ? Operator can be used in functions that return Result.
// Can't use in Main.

// To use ? Operator in your own functions,
// change the return type of your function to to be Result<T,E>
