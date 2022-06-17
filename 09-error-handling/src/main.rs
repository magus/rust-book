use std::error::Error;
use std::fs;
use std::io;

// use std::{
//     fs
//     fs::{self, File},
//     // io::{self, ErrorKind, Read},
//     io::{self, Read},
// };

// use std::io::ErrorKind;

// we can change the return of `main` to be `Result` that handles 'any error'
// this will exit with non-zero when error and 0 otherwise (`Ok(())`)
// for example, in the example below if we give an invalid `filepath` the return code is `101`

// small example of returning with exit code from main
// fn main() -> Result<(), Box<dyn Error>> {
//     let filepath = "does-not-exist.txt";
//     fs::read_to_string(filepath)?;

//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");

    // // index out of bounds: the len is 3 but the index is 99
    // let v = vec![1, 2, 3];
    // v[99];

    // giving a wrong type can give us an error showing type, i.e.  found enum `Result<File, std::io::Error>`
    // let f: i32 = File::open("hello.txt");
    // with vscode extension the result type is also shown anyway, i.e. Result<File, Error>
    // let f = File::open("hello.txt");

    let filepath = "hello.txt";

    // let f = match File::open(filepath) {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Problem creating [{}]: {:?}", filepath, error),
    //         },
    //         other_error => {
    //             panic!("Problem opening [{}]: {:?}", filepath, other_error);
    //         }
    //     },
    // };

    // we can use `unwrap_or_else` to clean up nested `match` expressions

    // let f = File::open(filepath).unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating [{}]: {:?}", filepath, error);
    //         })
    //     } else {
    //         panic!("Problem opening [{}]: {:?}", filepath, error);
    //     }
    // });

    // `unwrap` gives us the `panic!` when an error is encountered and the File otherwise
    // let f = File::open("hello.txt").unwrap();
    // using `expect` here to unwrap customizes the error message, making it easier to find in output
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    match read_username_from_file(filepath) {
        Ok(username) => println!("username: {}", username),
        Err(error) => panic!("unable to read username: {:?}", error),
    }

    Ok(())
}

// fn read_username_from_file(path: &str) -> Result<String, io::Error> {
//     let f = File::open(path);

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    // the `?` essentially handles the `Err` in the `match` scenario
    // propogating the error upward to the caller

    // let mut f = File::open(path)?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // can even shorten this code further by chaining method calls immediately after the `?`

    // let mut s = String::new();
    // File::open(path)?.read_to_string(&mut s)?;
    // Ok(s)

    // and even shorter still

    return fs::read_to_string(path);
}
