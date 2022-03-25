fn main() {
    println!("Hello, world!");

    unit_label(42, 'm');

    let four = {
        let x = 3;
        x + 1
        // implicit return x + 1
    };
    println!("four: {}", four);

    println!("power_level(): {}", power_level());

    let a = [1, 2, 3];
    let b = [1, 2, 3];
    println!("a == b: {}", a == b); // true! (array literals)

    println!("loop (i >= 10)");
    let mut i = 0;
    let iters = loop {
        println!("again! ({})", i);
        i += 1;

        if i >= 10 {
            break i;
        }
    };
    println!("iters: {}", iters);

    println!("while i < 10");
    let mut i = 0;
    while i < 10 {
        println!("again! ({})", i);
        i += 1;
    }

    println!("for-in array a");
    for element in a {
        println!("a[]: {}", element);
    }

    println!("for (1..10).rev()");
    for i in (1..10).rev() {
        println!("{}...", i);

        if i == 1 {
            println!("Liftoff!");
        }
    }
}

fn unit_label(x: i32, unit: char) {
    println!("{}{}", x, unit);
}

fn power_level() -> i32 {
    return 9_001;
}

// peaking ahead at documentation comments
// seems really neat, outputs to HTML and also can assert (tests!)
