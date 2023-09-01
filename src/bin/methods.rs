
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect: Rectangle = Rectangle {
        width: 10,
        height: 32,
    };

    println!("Old area: {}", rect.area());
    rect.inc_width(10);
    println!("New area: {}", rect.area());
}