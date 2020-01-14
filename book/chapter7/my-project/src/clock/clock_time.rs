use std::time::{SystemTime};
pub fn print_clock_time(){
	println!("The time is {:?}", SystemTime::now() );
}