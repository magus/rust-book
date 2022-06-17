fn main() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants can be evaluated
    // https://doc.rust-lang.org/reference/const_eval.html
    // e.g. the result of block statements which include flow control statements
    const ONE_OR_TWO: i8 = if true {
        // force block
        1
    } else {
        2
    };
    println!("The value of ONE_OR_TWO is: {}", ONE_OR_TWO);

    // shadow the x from earlier
    let x = "hi";
    {
        let x = "hello";
        println!("The scoped value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

    // really feels weird to allow shadowing,
    // it totally breaks the 'if not mut then variable won't change' assumption introduced at the beginning of the chapter
    // requires reading every line to see if its reassigned...
    // would you think this is the kind of style thing you'd want to just avoid? why/why not

    // parsing data types

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // thread 'main' panicked at 'Not a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:34:35
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    // ran `RUST_BACKTRACE=1 cargo run`
    //
    // thread 'main' panicked at 'Not a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:34:35
    // stack backtrace:
    // 0: rust_begin_unwind
    //             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/std/src/panicking.rs:498:5
    // 1: core::panicking::panic_fmt
    //             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/panicking.rs:107:14
    // 2: core::result::unwrap_failed
    //             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/result.rs:1613:5
    // 3: core::result::Result<T,E>::expect
    //             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/result.rs:1255:23
    // 4: variables::main
    //             at ./src/main.rs:34:22
    // 5: core::ops::function::FnOnce::call_once
    //             at /rustc/db9d1b20bba1968c1ec1fc49616d4742c1725b4b/library/core/src/ops/function.rs:227:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // let guess: u32 = "hi".parse().expect("Not a number!");
    // println!("guess: {}", guess);

    // number literal
    let numlit = 42u16; // specify type as suffic, i.e. let numlit: u16 = 42;
    println!("numlit: {}", numlit);
    let numlit = 9_000 + 1; // _ thousands separator
    println!("numlit: {}", numlit);
    let numlit = 123; // default i32
    println!("numlit: {}", numlit);

    // floating point types

    // f64 by default

    // classic IEEE-754 standard floating thing (existing in most languages)
    println!("0.1 + 0.2: {}", 0.1 + 0.2); // 0.30000000000000004
    let x: f32 = 16777217.1;
    println!("(f32) 16777217.1: {}", x); // 16777218

    // integer division as expected
    let parts = 5 / 2;
    println!("5 / 2: {}", parts); // 2

    // chars

    // rust chars are 4 byte unicode values so they can contain that full space
    // e.g. emojis, chinese characters, etc.
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    let lol: char = '😂';
    println!("lol: {}", lol);

    // tuples
    let tup = (500, 6.4, -7);
    let (x, y, z) = tup;
    println!("x,y,z: {},{},{}", x, y, z);
    println!("tup.0,tup.1,tup.2: {},{},{}", tup.0, tup.1, tup.2);
    // error: cannot be printed because there is no default formatter for this specific tuple type (integer, float, integer)
    // println!("tup: {}", tup);

    // arrays

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {}", a[2]);
    let zeroes = [0; 100];
    println!("zeroes.length: {}", zeroes.len());
    let [a1, a2, _, _, _] = a; // must destructure entire array not just some elements
    println!("a1,a2: {},{}", a1, a2);
}
