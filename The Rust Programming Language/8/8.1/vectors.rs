fn main() {
    let v: Vec<i32> = Vec::new();
}

fn main() {
    let v = vec![1, 2, 3];
}

fn main() {
    let mut v = Vec::new();

    v.push(5);
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}
