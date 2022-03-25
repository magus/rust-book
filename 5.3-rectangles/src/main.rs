#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }

    fn random() -> u32 {
        return 42;
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn grow(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect.area()={}", rect.area());

    rect.grow();
    println!("rect.grow(); rect.area()={}", rect.area());

    let rect_a = Rectangle {
        width: 1,
        height: 200,
    };
    let rect_b = Rectangle {
        width: 200,
        height: 100,
    };
    let rect_c = Rectangle {
        width: 9001,
        height: 9001,
    };

    println!("rect_a.can_hold(&rect_b)={}", rect_a.can_hold(&rect_b));
    println!("rect_b.can_hold(&rect_a)={}", rect_b.can_hold(&rect_a));
    println!("rect_c.can_hold(&rect_a)={}", rect_c.can_hold(&rect_a));

    let square = Rectangle::square(3);
    dbg!(&square);

    println!("Rectangle::random()={}", Rectangle::random());
}
