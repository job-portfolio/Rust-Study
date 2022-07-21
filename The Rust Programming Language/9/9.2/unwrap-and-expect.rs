use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}

// expect allows us to provide a customised error message.
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}


/*
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
*/

