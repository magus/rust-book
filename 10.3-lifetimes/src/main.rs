// fn main() {
//     {
//         let r;

//         {
//             let x = 5;
//             r = &x;
//         }

//         println!("r: {}", r);
//     }
// }

use std::fmt::Display;

const STATIC_VALUE: &i32 = &42;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("longest(string1.as_str(), string2) = {}", result);
    let result = longest_with_an_announcement(string1.as_str(), string2, "Longest found!");
    println!(
        r#"longest_with_an_announcement(string1.as_str(), string2, "Longest found!") = {}"#,
        result
    );

    // // playing with the scope after specifying the lifetimes
    // // we can see the borrow checker will enforce the consistent (smaller)
    // // lifetime since we used a single lifetime it must be the shorter of the two
    // // `string2` does not live long enough
    // //    borrowed value does not live long enough
    // let string1 = String::from("long string is long");
    // let result: &str;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // be careful the static lifetime means this reference is available for life of program
    // we should only use this if that is actually true!
    // for example, that's not true here, bad, don't do this!
    // let static_str: &'static str = "hello world";

    let static_i32 = STATIC_VALUE;
    println!("static_i32   = {:>3} ({:p})", static_i32, static_i32);
    println!("STATIC_VALUE = {:>3} ({:p})", STATIC_VALUE, STATIC_VALUE);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
