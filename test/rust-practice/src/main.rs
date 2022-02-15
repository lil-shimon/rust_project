mod ownership;
mod generics;
mod lifetime;
mod structs;

fn main() {
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();

    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana)
}

trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;

impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;

impl Fruits for Banana {
    fn price(&self) -> u32 {
        30
    }
}

fn get_price<T: Fruits>(fruits: T) {
    println!("price is: {}", fruits.price())
}