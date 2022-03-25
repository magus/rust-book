#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // structs
    // ==================

    let user_a = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    println!("(debug) user_a={:#?}", user_a);

    print_user(&user_a);

    let mut user_b = User {
        email: String::from("another@example.com"),
        ..user_a
    };

    // Error! we cannot do this now since user_a String are borrowed during struct update
    // print_user(&user_a);

    user_b.active = false;

    print_user(&user_b);

    // Error! cannot use &str here since it cannot be owned by struct
    // requires lifetime specifiers
    // Chp 10
    //
    // struct UserStringSlice {
    //     active: bool,
    //     username: &str, // missing lifetime specifier, expected named lifetime parameter
    //     email: &str, // missing lifetime specifier, expected named lifetime parameter
    //     sign_in_count: u64,
    // }
    // let user_slicestrings = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };

    // tuple structs
    // ==================

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);

    struct UnitStruct;
    let unit = UnitStruct; // `UnitStruct {}` implied

    println!("red=({},{},{})", red.0, red.1, red.2);
    println!("origin=({},{},{})", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!(
        "{} ({}, active={}, sign_in_count={})",
        user.username, user.email, user.active, user.sign_in_count
    );
}
