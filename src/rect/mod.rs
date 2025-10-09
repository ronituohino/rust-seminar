#[derive(Debug, Clone)]
struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T> {
    fn new(width: T, height: T) -> Rectangle<T> {
        Rectangle { width, height }
    }
}

impl<T> Rectangle<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn test() {
    for i in 0..=3 {
        println!("Iteration: {}", i);
    }

    let name = "Alice";

    println!("Hello, {}!", name);

    println!("{} - {}", add(2, 3), name);

    let x = 5;
    let y = &x;

    let z = *y + 10;

    println!("x: {}, y: {}, z: {}", x, *y, z);

    let some: Option<i32> = None;

    let res = match some {
        Some(v) => v + 10,
        None => 0,
    };

    println!("Result: {}", res);

    let r1: Rectangle<i32> = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", r1.area());

    let r2 = Rectangle::new("ten", "twenty");
    println!("Rectangle r2: {:?}", r2);
}
