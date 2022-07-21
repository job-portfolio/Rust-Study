#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    
    fn area(&self) -> u32 {         // &self is actually short for self: &Self
        self.width * self.height    // fn area(self: &Self) -> u32 { 
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

/*
In the signature for area, we use &self instead of rectangle: &Rectangle.
The &self is actually short for self: &Self.
Within an impl block, the type Self is an alias for the type that the impl block is for.
Methods must have a parameter named self of type Self for their first parameter,
so Rust lets you abbreviate this with only the name self in the first parameter spot.
Note that we still need to use the & in front of the self shorthand to indicate this
method borrows the Self instance, just as we did in rectangle: &Rectangle. Methods can
take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably,
just as they can any other parameter.
*/
