macro_rules! yo {
 ($name:expr) => {
    println!("Yo, {}!", $name);
 };
}

fn main() {
    println!("Hello, world!");
    yo!("a");
}
