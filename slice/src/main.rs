fn main() {
    let mut s = String::from("hello world");
    let word = first_word_index(&s);
    println!("{}", word);
    // This will clear the string by replacing it with an empty string ""
    // But the word is still valid & holds the value of 5 even though the string is cleared
    // If we try to extract the first word using 5 after this line it will result in an error
    // That means returned index from first word is meaningful only in the context of the string
    s.clear();

    let mut s = String::from("hello world");
    // Creating a "String Slice"
    let hello = &s[0..5];
    // The above line could also be written as
    // If the slice starts at index 0, the starting index in range can be omitted.
    let hello = &s[..5];

    let world = &s[6..11];
    // The above line could also be written as
    // If the slice ends at the end of the string, the ending index in range can be omitted.
    let world = &s[6..];

    // You can drop both the indices in the range to get the full string.
    let hello_world = &s[..];

    println!("{} {}", hello, world);
    println!("{}", hello_world);

    let word = first_word(hello_world);
    println!("{}", word);

    // This is a string literal
    // String literals are slice pointing to specific point of binary
    // This is why string literals are immutable
    let s = "hello world";

    let arr = [1, 2, 3, 4, 5];
    // Creating a slice of the i32 array
    let slice = &arr[1..3];

}

// ineffective way to get the first word
// can later introduce bugs if not careful
fn first_word_index(str: &String) -> usize {
    // Converting string to byte slice
    let bytes = str.as_bytes();

    // Iterating over the byte slice
    // Returns the index of the first space
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }

    // Returns the length of the string if no space is found
    str.len()
}

// The correct way to do this is
// fn first_word(str: &String) -> &str {
// The below function signature is better than above since
// it allows us to work with both &String value and &str value
fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..index];
        }
    }

    &str[..]
}
