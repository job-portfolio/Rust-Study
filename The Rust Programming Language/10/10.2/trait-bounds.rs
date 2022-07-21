
pub trait Summary {
    fn summarize(&self) -> String;
}

// Two way to write traits as a function parameter

// Option 1:
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Option 2:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

