
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Multiple impl blocks are allowed in rust for the same type
impl Rectangle {
    // &self is short for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods can have same name as that of the fields/properties
    // Rust would identify method/property one to use based on if it includes parentheses or not
    // For eg: width() would be method vs width would be property
    // methods that have same name as fields/properties are called getters
    // they are used to get the value of the field
    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // All functions in the impl block are called associated functions
    // Associated functions that doesn't take self are not considered as methods
    // They are generally used for constructors (generally called "new")
    // The "Self" keyword is used as an alias for the type of the struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect_one = Rectangle {
        width: 30,
        height: 50,
    };

    // Both below lines are equivalent
    // for method calls rust has automatic dereferencing
    rect_one.area();
    (&rect_one).area();

    let rect_two = Rectangle {
        width: 10,
        height: 40,
    };

    println!("The area of the rectangle is {}", rect_one.area());
    println!("Can rect_one hold rect_two? {}", rect_one.can_hold(&rect_two));
    println!("Can rect_two hold rect_one? {}", rect_two.can_hold(&rect_one));

    // To call non-method associated functions we need to use "::" followed by the function name
    let square = Rectangle::square(3);
}