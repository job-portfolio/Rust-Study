use std::fmt; // Import `fmt`

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point3D {
    real: f64,
    image: f64,
}

// Similarly, implement `Display` for `Point3D`
impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{}, {}", self.real, self.image)
    }
}

fn main() {
    let point = Point3D { real: 3.3, image: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}


// Display: 3.3 + 7.2i
// Debug: Complex { real: 3.3, imag: 7.2 }