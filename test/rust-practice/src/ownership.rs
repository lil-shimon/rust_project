pub fn run() {
    let s1 = String::from("hello"); // s1が所有権を持っている
    let s2 = s1; // s1からs2に所有権を渡している
    println!("{}", s2); // s1は所有権がないからエラー (value borrowed here after move)

    let i1 = 1;// i1が所有権を持っている
    let i2 = i1; // i2にコピー (heapにデータを保存しないタイプはコピーできる)
    println!("{}, {}", i1, i2); // i1とi2が表示される

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{}, {}", sl1, sl2);

    let s3 = String::from("string deep copy"); // String型。所有権を持っている
    let s4 = s3.clone(); // s3をdeep copyしている
    println!("{}, {}", s3, s4);
    println!("s3のスタックアドレスは {:p}", &s3);
    println!("s4のスタックアドレスは {:p}", &s4);
    println!("Heap memory of address of s3 {:p}", s3.as_ptr());
    println!("Heap memory of address of s4 {:p}", s4.as_ptr());

    let s5 = String::from("hello");
    print_var_info(&s5);
    take_ownership(s5);
    let s6 = String::from("hello"); // ownership
    print_var_info(&s6);
    let s7 = take_giveback_ownership(s6); // pass ownership to s7
    print_var_info(&s7);
    let mut hello = String::from("hello");
    change(&mut hello);
    print_var_info(&hello);
}

fn take_ownership(s: String) {
    print_var_info(&s);
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn print_var_info(s: &String) {
    println!("stack address is {:p}", &s); // stack address
    println!("heap address is {:?}", s.as_ptr()); // heap address
    println!("length is {}", s.len()); // length
    println!("capacity is {}", s.capacity()); // capacity
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world")
}