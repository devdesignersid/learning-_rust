fn main() {
    // Here "Hello, World!" is a string literal.
    // String literal like this cannot be mutated .i.e they are immutable.
    // Can only be used when the values are known .
    // String literals are stored in the read-only memory segment (not on stack or on heap).
    // Since they are hardcoded they are known at compile time.
    "Hello, World!";

    // Strings are stored in heap because they often
    // require more flexibility than what stack can provide.
    // They can have dynamic length.
    // The stack is designed for fixed size known-at-compile-time data.
    // The stack has limited size of memory to be more fast and efficient.
    // Large or enormous strings can exceed the stack size.
    // String allocated on heap can have lifetime that outlives the function.
    // This is harder to manage on stack, which is tied to the function scope.
    let mut str = String::from("hello");
    str.push_str(", world!");
    println!("{}", str);

    let s1 = String::from("hello");
    // Transferring ownership of s1 to s2
    // After this line s1 will no longer be valid
    // Accessing s1 after this line will result in an error
    // This is because s1 & s2 are both pointers to the same location in memory (heap).
    // if both s1 & s2 goes out of scope, then rust compiler would free the memory twice calling the drop method twice.
    // De-allocating the memory twice would result in double free error.
    // Freeing memory twice can cause memory corruption which can result in security vulnerabilities.
    // Making the copy of the pointer, length and capacity of s1 to s2 and invalidating s1 is called "move".
    let s2 = s1;

    let s1: String = String::from("hello");
    // .clone does a deep copy rather than copying the reference.
    // Now both s1 & s2 points to two different locations in memory(heap).
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    // Rather than passing ownership of s to calculate_length
    // we are passing a reference to s to calculate_length
    // '&' is a reference operator
    calculate_length(&s);

    let mut s = String::from("hello");
    // We are passing a mutable reference to s
    // '&mut' is a mutable reference operator
    // You can only have one mutable reference to a particular piece of data in a particular scope
    // So let s1 = &mut s; let s2 = &mut s; will not compile
    // This compiler prevents this in order to prevent data races
    change(&mut s);
    println!("{}", s);

    // We cannot have a mutable reference while we have an immutable reference pointing to the same data
    // The below code will not compile
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{}, {}, and {}", r1, r2, r3);

    // Mutable references after the reference scope of the variable can exist even though immutable references exist
    // The reference scope starts when the reference is created until the reference is last used
    // This will compile perfectly
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // let r3 = &mut s;
    // println!("{}", r3);

    // The rust compiler guarantees preventing any dangling references
    // Heres an example of a dangling reference
    // The reference will be dropped at the end of the
    // function scope so its pointing to something that no longer exists
    // Rust wont allow compiling the below code
    // let reference_to_nothing = dangle();
}

// Reference to s is passed to this function
// So when the function scope ends the value of s will not be dropped
// s is still valid, but the reference will be dropped
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Receiving a mutable reference means the value of s might be changed / mutated
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
     */
