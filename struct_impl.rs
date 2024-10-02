struct Rect {
    area: i32,
    width: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.area * self.width
    }
}

fn main() {
    let rect1 = Rect {
        area: 20,
        width: 20,
    };

    println!("The area of rectangle is {}", rect1.area());
}