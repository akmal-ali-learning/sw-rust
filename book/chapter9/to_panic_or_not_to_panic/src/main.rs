fn main() {

	//When to use Panic!

	//!< In prototype code.

	//Calling unwrap or expect are useful
	// Don't need to decide how to handle errors.
	// panic! is how a test is marked as a failure.

	// Other cases, when you have some logic that ensures Result will have an Ok value.
	// You can call unwrap as you assume logic  

	// PANIC!!!!
	// Should panic when it's possible that your code could end up in a bad state. 
	// When an assumption, guarantee, contract or invariant has been broken.
	// When invalid, contradictory or missing values are passed to your code.



}

// Creating custom types for validation.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}