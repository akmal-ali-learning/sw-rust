fn main() {
    println!("Hello, world!");

    let x = 1;
    let y = add_one::add_one(x);
    let y = add_two::add_two(y);
    assert_eq!(y ,x+3 );
}
