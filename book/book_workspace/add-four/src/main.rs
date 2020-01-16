
/**
 * @brief      To run this program from the Cargo workspace, Must do cargo run -p add-four
 */
fn main() {
    println!("Hello, world!");

    let x = 1;
    let y = add_two::add_two(x);
    let y = add_two::add_two(y);
    assert_eq!(y ,x+4 );
}