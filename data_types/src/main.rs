fn main() {
    // Scalar Types

    // Integers
    // Default is "i32" (32-bit signed integer)
    // Can contain both positive and negative numbers
    // i32 has a range of -2,147,483,648 to 2,147,483,647  (-2^32 to 2^32 - 1)
    let x = -5;
    println!("The value of x is: {}", x);

    // Unsigned 32-bit integers
    // u32 has a range of 0 to 4,294,967,295 (0 to 2^32 - 1)
    let y: u32 = 5;
    println!("The value of y is: {}", y);

    // Floating point numbers
    // Default is f64
    // All Floating point numbers are signed
    let z = 2.0; // f64 (64-bit floating point)
    println!("The value of z is: {}", z);
    let z: f32 = 3.0; // f32 (32-bit floating point)
    println!("The value of z is: {}", z);

    // Numeric Operations
    let sum = 5 + 10;
    println!("The sum is: {}", sum);
    let difference = 95.5 - 4.3;
    println!("The difference is: {}", difference);
    let product = 4 * 30;
    println!("The product is: {}", product);
    let quotient = 56.7 / 32.2;
    println!("The quotient is: {}", quotient);
    let remainder = 43 % 5;
    println!("The remainder is: {}", remainder);

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    // Characters
    let c = 'z';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // Compound Types

    // Tuples
    // Tuples can have different types
    // Tuples can be nested
    // Tuples are useful when you want to return multiple values from a function
    // Tuples have a fixed length
    // Type annotations are optional
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);
    // Access tuple elements using indexing
    println!("The value of y is: {}", tup.1);
    // Empty Tuples are called "unit" tuples
    // Unit tuples have a length of 0
    // let unit_tup = ();

    // Arrays
    // Arrays have fixed length
    // All elements should have the same type
    // Array memory is allocated on the stack
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // Access array elements using indexing
    println!("The value of months[0] is: {}", months[0]);
    // Define an array with a fixed length and data type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Defining an array with same item for a given length
    let a = [3; 5]; // [3, 3, 3, 3, 3]

}
