pub fn add_two(x: i32) -> i32 {

    let x = add_one::add_one(x);
    let x = add_one::add_one(x);
    x
}


// In order to run this particular test from the workspace, run cargo test -p add-two.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
