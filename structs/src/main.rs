struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
};

// Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit Like Struct
// They behave similarly to () a.k.a unit type
struct AlwaysEqual;

fn main() {
    // Create a new mutable user instance
    let mut user = build_user(String::from("user@example.com"), String::from("username"));

    // Create a new user instance with specific values
    let new_user = User {
        active: true,
        username: String::from("new_username"),
        email: String::from("new_email@example.com"),
        sign_in_count: 1,
    };

    // Struct update syntax
    // We use "..user" to copy the values from `user` for any fields we don't specify.
    // `username` is of type `String`, which is not `Copy`, so it is moved to `new_user`,
    // meaning `user` can no longer be used for its `username` field.
    // However, `active` (bool) and `sign_in_count` (u64) are `Copy`, so they are duplicated,
    // not moved, and `user` can still be used for those fields.
    let new_user = User {
        email: String::from("newuseremail@example.com"),
        ..user // Copies `active` and `sign_in_count` from `user` and moves `username`
    };

    // Updating the email field in `user`
    // The entire instance must be mutable, as Rust does not allow making only certain fields mutable.
    user.email = String::from("newemail@example.com");

    // Using tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Usage is similar to tuples
    println!("{} {} {}", black.0, black.1, black.2);

    // Instantiating Unit struct
    let subject = AlwaysEqual;
}

// Function to build a new User instance
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,      // shorthand for username: username
        email,         // shorthand for email: email
        sign_in_count: 1,
    }
}
