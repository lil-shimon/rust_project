// 普通だと構造体はデータが複雑なため、コンソールに表示はできないが、
// #[derive(Debug)]を使えば表示できる
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height)
    }
}

pub fn run() {
    let user1 = User {
        username: String::from("admin"),
        email: String::from(String::from("admin@example.com")),
        sign_in_count: 1,
        active: true,
    };
    let mut user1 = User {
        username: String::from("admin"),
        email: String::from("admin@example.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("sub@example.com");
    println!("{:#?}", user1);
}
