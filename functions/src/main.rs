fn main() {
    another_function(5, true);

    // This is a statement
    // Statements do not return a value
    // Can't write let x = (let y = 6);
    // Because unlike other programming languages, rust does not return values on assignments
    // In other programming languages, we can write let x = y = 6;
    let y = 6;
    println!("The value of y is: {}", y);

    // Expressions evaluate to a value
    // Expressions can be part of statements
    // Calling a function / Calling macro would also be expressions
    let z = {
        let x = 3;

        // Expressions doesn't end with a semicolon
        // If we add a semicolon, it will be a statement
        x + 1
    };
    println!("The value of z is: {}", z);

    let sum = add(3, 5);
    println!("The value of sum is: {}", sum);

    do_nothing();
}

// using snake_case for function names
fn another_function(x: i32, y: bool) {
    println!("The value of x is: {} and this is {}", x, y);
}

// function that returns a value
// return type is specified using an arrow ->
// will return the last expression
// can return early using return keyword with a value
fn add(x: i32, y: i32) -> i32 {
    x + y
}
// function that does nothing
// it return nothing specified using an arrow -> '()' an empty tuple
// the empty tuple also called a unit type means it has no values to return
fn do_nothing() -> () {}
