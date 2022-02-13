pub fn run() {
    let s1 = String::from("hello"); // s1が所有権を持っている
    let s2 = s1; // s1からs2に所有権を渡している
    println!("{}", s2); // s1は所有権がないからエラー (value borrowed here after move)

    let i1 = 1;// i1が所有権を持っている
    let i2 = i1; // i2にコピー (heapにデータを保存しないタイプはコピーできる)
    println!("{}, {}", i1, i2); // i1とi2が表示される
}