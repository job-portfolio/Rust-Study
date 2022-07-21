fn main() {
    // completes
    let spaces = "   ";
    let spaces = spaces.len();

    // error
    let mut spaces = "   ";
    spaces = spaces.len();

    /*
    $ cargo run
    Compiling variables v0.1.0 (file:///projects/variables)
    error[E0308]: mismatched types
    --> src/main.rs:3:14
    |
    2 |     let mut spaces = "   ";
    |                      ----- expected due to this value
    3 |     spaces = spaces.len();
    |              ^^^^^^^^^^^^ expected `&str`, found `usize`

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `variables` due to previous error
    */
}
