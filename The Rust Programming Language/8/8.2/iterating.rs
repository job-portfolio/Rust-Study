
#![allow(unused)]
fn main() {
for c in "नमस्ते".chars() {
    println!("{}", c);
}
}

/*
न
म
स
्
त
े
*/

// ----


#![allow(unused)]
fn main() {
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
}

/*
224
164
// --snip--
165
135
*/