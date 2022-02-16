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
    get_price(banana);

    let res = div_opt(4.0, 2.0);
    match res {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Not allowed!")
    }

    let res_1 = div_result(20.0, 3.9);
    match res_1 {
        Ok(x) => println!("Result: {:.3}", x),
        Err(e) => println!("{}", e)
    }
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

fn div_opt(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn div_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not allowed"))
    } else {
        Ok(x / y)
    }
}