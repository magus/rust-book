#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 50;
    let height = 30;

    // cool!
    // commenting out this line causes width and height above to be i32 instead of u32
    // passing them into area hints the compiler and instantiates them as u32
    println!("area={}px^2", area(width, height));

    // better to use a struct to convery meaning instead of unlabeled parameters
    let scale = 3;
    let mut rectangle = Rectangle {
        width: dbg!(50 * scale),  // > [src/main.rs:19] 50 * scale = 500
        height: dbg!(30 * scale), // > [src/main.rs:20] 30 * scale = 300]
    };
    println!("area_rect={}px^2", area_rect(&rectangle));

    // pretty print (Debug trait)
    println!("(debug) rectangle={:?}", rectangle);
    println!("(debug) rectangle={:#?}", rectangle);

    // dbg! macro prints to stderr (not stdout like println!)
    dbg!(&rectangle);

    rectangle.height = 50;
    println!("area_rect={}px^2", area_rect(&rectangle));
    dbg!(&rectangle);
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_rect(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
