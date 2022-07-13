extern crate foo;

fn main() {
    foo::try();
}


// You'll get this error:

/*
error: expected identifier, found keyword `try`
 --> src/main.rs:4:4
  |
4 | foo::try();
  |      ^^^ expected identifier, found keyword
*/


// You can write this with a raw identifier:

/*
extern crate foo;

fn main() {
    foo::r#try();
}
*/