fn pick_one<T>(a: T, b: T) -> T {
    println!("Test: {}", std::process::id());
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}

fn main() {
    println!("Coin toss: {}", pick_one("heads", "tails"));
    println!("Cash prize: {}", pick_one(100, 1000));
}