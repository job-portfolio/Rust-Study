/*
Each Rust source file in the tests directory is compiled as a separate crate. One way of sharing some code between integration tests is making a module with public functions, importing and using it within tests.
*/

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
}

// importing common module.
mod common;

#[test]
fn test_add() {
    // using common code.
    common::setup();
    assert_eq!(adder::add(3, 2), 5);
}
