
#![allow(unused)]
fn main() {
    let s = String::from("hello");
    let len = s.len();

    // ALL THESE ARE EQUAL

    let slice = &s[0..2];
    let slice = &s[..2];
    

    let slice = &s[3..len];
    let slice = &s[3..];


    let slice = &s[0..len];
    let slice = &s[..];
}
