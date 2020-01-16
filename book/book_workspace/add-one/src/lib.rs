pub fn add_one(x: i32) -> i32 {
    x + 1
}


// In order to run this particular test from the workspace, run cargo test -p add-one.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }
}
