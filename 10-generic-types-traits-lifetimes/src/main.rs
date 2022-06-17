use std::fmt::Display;

fn main() {
    let list = vec![34, 50, 25, 100, 65];
    println!("largest_i32(&list) = {}", largest_i32(&list));
    println!("largest(&list) = {}", largest(&list));
    println!("largest_ref(&list) = {}", largest_ref(&list));
    let list = vec!['a', 'g', 'w', 'k', 'c'];
    println!("largest_char(&list) = {}", largest_char(&list));
    println!("largest(&list) = {}", largest(&list));
    println!("largest_ref(&list) = {}", largest_ref(&list));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer: Point<i32> = {:?}", integer);
    println!("float: Point<f64> = {:?}", float);

    // distance_from_origin only defined for Point<f64, f64>
    // let wont_work = integer.distance_from_origin();
    let does_work = float.distance_from_origin();
    println!("Point<i32, f64>.distance_from_origin() = {:?}", does_work);

    let int_and_float = Point { x: 5, y: 4.0 };
    println!("Point<i32, f64> = {:?}", int_and_float);

    println!("Point<i32, f64>.x() = {:?}", int_and_float.x());
    println!("Point<i32, f64>.y() = {:?}", int_and_float.y());

    let mixup = integer.mixup(&float);
    println!("! integer.mixup(float) = {:?}", mixup);
    // cannot access integer or float after implicit borrow
    // println!("integer = {:?}", integer);
    // println!("float = {:?}", float);

    integer.cmp_display();
    // cannot call because types are mismatched
    // mixup.cmp_display();
}

#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    fn x(&self) -> &X {
        &self.x
    }

    fn y(&self) -> &Y {
        &self.y
    }
}

impl<X, Y> Point<X, Y> {}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// we can rename generics when impl since it's a declaration
// this can be helpful for readability, like below
// we are referencing another Point X2 & Y2 in parameters of mixup fn
impl<X1: Copy, Y1: Copy> Point<X1, Y1> {
    fn mixup<X2: Copy, Y2: Copy>(&self, other: &Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// use trait bound to define a method for specific types with specific traits
impl<T> Point<T, T>
where
    T: Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x ({}) > y ({})", self.x, self.y);
        } else {
            println!("y ({}) > x ({})", self.y, self.x);
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = match &list[0] {
        &value => value,
    };

    // pattern matching ... again MAGICAL 🧙‍♂️✨
    // the type of `number` is `&i32`
    // so the pattern match is
    //    `&number` === `&i32`
    // so it figures out that `number` must be `i32`

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest_ref<T: PartialOrd + Copy>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest_i32(list: &Vec<i32>) -> i32 {
    let mut largest = list[0];

    // pattern matching ... again MAGICAL 🧙‍♂️✨
    // the type of `number` is `&i32`
    // so the pattern match is
    //    `&item` === `&i32`
    // so it figures out that `number` must be `i32`
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest_char(list: &Vec<char>) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
