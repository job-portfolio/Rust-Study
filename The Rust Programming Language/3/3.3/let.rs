fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let p = {let q = 6; q+1};
    println!("The value of p is: {p}");
}