
#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddressKindV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpV4Addr {}
struct IpV6Addr {}

// Illustrates how enum can hold any type of data
enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

// Methods can be defined on enums just like structs using the "impl" keyword
impl Message {
    fn call(&self) {}
}


fn main() {

    // Creating instances of enum
    // We use "::" to call the enum constructor
    // We automatically get constructor function result of defining the enum
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    println!("four: {:?}", four);
    println!("six: {:?}", six);

    // enums can contain data of different types
    // "four_v2" contains a tuple
    // "six_v2" contains a String
    let four_v2 = IpAddressKindV2::V4(127, 0, 0, 1);
    let six_v2 = IpAddressKindV2::V6(String::from("::1"));
    println!("fourV2: {:?}", four_v2);
    println!("sixV2: {:?}", six_v2);

    let msg = Message::Write(String::from("hello world"));
    msg.call();

    // Rust doesn't have "null" type.
    // The same concept can be implemented using Option enum
    // Option<T> enum has two variants: Some and None
    // Some: Contains a value
    // None: Does not contain a value
    // <T> is a generic type
    // T can be any type

    // For Some, the type of T can be inferred
    let some_number = Some(5);
    let some_char = Some('e');

    // For None, the type of T should be explicitly specified
    let absent_number: Option<i32> = None;

    // This will result in a compilation error
    // This is because some_number is of type Option<i32>
    // Rust doesn't know how to add 2 to Option<i32>
    // let sum = some_number + 2;
}
