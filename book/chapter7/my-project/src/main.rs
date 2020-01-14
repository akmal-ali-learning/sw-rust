use rand::{self,Rng};
mod clock;

pub use crate::clock::clock_time;

fn main() {
	clock_time::print_clock_time();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number {} ", secret_number );
    clock_time::print_clock_time();
}