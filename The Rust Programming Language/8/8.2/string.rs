fn main() {
    let mut s = String::new();
}

fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}

fn main() {
    let s = String::from("initial contents");
}

fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
}

fn main() {
    let mut s = String::from("lo");
    s.push('l');
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}