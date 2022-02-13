pub fn run() {
    let s1 = String::from("hello"); // s1が所有権を持っている
    // s1からs2に所有権を渡している
    let s2 = s1;
    println!("{}", s2)
    // s1は所有権がないからエラー (value borrowed here after move)
}