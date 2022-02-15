pub fn run() {
    let string_one = String::from("a");
    let string_two = String::from("y");
    let response = get_longest(&string_one, &string_two);
    println!("{}", response)
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
