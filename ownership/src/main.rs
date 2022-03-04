fn main() {
    let mut s = String::from("hello");

    // push_str() appends a literal to a String
    s.push_str(", world!");

    // r#""#
    // ^^^^^ this is a rust string literal
    println!(r#"s = "{}""#, s);

    let mut x = 5;
    let y = x;
    x = x + 2;
    println!("(x,y) = ({},{})", x, y);

    // playing with moving and ownership

    let s1 = String::from("hello");

    // s1 is moved to s2
    // s1 no longer valid
    let _s2 = s1;

    // this code throws borrow errors, cool!
    // seems ownership related like s2 takes ownership over s1 memory, wild
    // s1.push_str(" some changes...");
    // we cannot even access s1, much less change it
    // println!(r#"s1 = "{}" s2 = "{}""#, s1, s2);

    // reassigning works because we reestablish ownership over new segment in heap memory
    // s1 = String::from("hello");

    // a cool guarantee of the move behavior is that we never deep copy
    // > "any automatic copying can be assumed to be inexpensive in terms of runtime performance."

    let s1 = String::from("string_one");
    let s2 = s1.clone();
    println!(r#"s1 = "{}" s2 = "{}""#, s1, s2);

    // we can call clone on primitive values too... wowa.
    // it's superfluous though since it's clone by default
    let x = 5;
    let y = x.clone();
    println!("(x,y) = ({},{})", x, y);

    // do arrays implement Copy?
    // > Yes!
    let a = [1, 2, 3];
    let mut b = a;
    b[0] = b[0] + 10;
    println!("a[0] = {}", a[0]);
    println!("b[0] = {}", b[0]);

    // playing with function taking ownership
    let s = String::from("abcdefg");
    take_ownership(s);
    // error! cannot access s anymore
    // println!(r#"s = "{}""#, s);

    let s9 = String::from("abcdefg");
    // error! the return doesn't really return ownership, we still have to reassign
    // return_ownership(s9);
    // println!(r#"s = "{}""#, s9);
    let s10 = return_ownership(s9);
    println!(r#"s = "{}""#, s10);

    let s = String::from("how long am i?");
    println!("calculate_length(&s) = {}", calculate_length(&s));

    let mut s = String::from("a");

    // error! cannot create two mutable refs
    // let s111 = &mut s;
    // let s222 = &mut s;
    // println!("{}, {}", s111, s222);

    // we can do this though because the mutable reference `&mut s` is used and returned
    // before the `mutate_reference` is called for the second time
    println!("mutate_reference(&s) = {}", mutate_reference(&mut s));
    println!("mutate_reference(&s) = {}", mutate_reference(&mut s));

    let s = String::from("return first word");
    println!(r#"first_word_index("{}") = {}"#, s, first_word_index(&s));

    let mut _s = String::from("what happens when i slice this?");
    let slice = first_word_slice(&s);
    // error! we cannot modify s anymore since we have a slice of it!
    // s.clear();
    println!(r#"first_word_slice("{}") = {}"#, s, slice);
}

fn take_ownership(s: String) {
    println!(r#"take_ownership("{}")"#, s);
}

fn return_ownership(s: String) -> String {
    println!(r#"return_ownership("{}")"#, s);
    s
}

fn calculate_length(s: &String) -> usize {
    // error! we can't mutate, nice!
    // s.push_str("adding some stuff that will change the length");
    // error! we can't reassign, nice!
    // s = String::from("can we make it another string?");

    // return the length of the referenced string
    s.len()
}

fn mutate_reference(s: &mut String) -> usize {
    s.push_str("adding some stuff that will change the length");

    // return the length of the referenced string
    s.len()
}

// error! cannot return reference since it would be cleanup up when function block is over
// fn dangle() -> &mut String {
//     let s = String::from("hello"); // s is a new String
//     &mut s
// }

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for i in 0..bytes.len() {
        let item = bytes[i];
        if item == b' ' {
            return i;
        }
    }

    // alternative using iter

    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }

    s.len()
}

// we can simplify `(s: &String)` to `(s: &str)`
// fn first_word_slice(s: &String) -> &str {
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for i in 0..bytes.len() {
        let item = bytes[i];
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}
