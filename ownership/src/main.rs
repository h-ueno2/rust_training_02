fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s); // sの値は関数にムーブしているため、有効じゃなくなる。
    // println!("{}", s); // これはコンパイルエラーになる。

    let x = 5;
    makes_copy(x); // i32はCopyなのでxはまだ使用可能。
    println!("{}", x); // これはコンパイルエラーにならない

    let s1 = gives_ownership(); 
    println!("{}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2); // これはコンパイルエラーになる。所有権が関数に移動しているため。
    println!("{}", s3);

    // `&`を付けることで所有権を関数に渡さない。
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of `{}` is {}.", s1, len);
    
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // 以下はドキュメントだとエラーになるって書いてあるけど
    // コンパイルできてしまう。
    let mut st = String::from("hello");
    let r1 = &st;
    let r2 = &st;
    let r3 = &mut st;
    r3.push_str("a");

    let reference_to_nothing = dangle();

    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let hello = &s[..5];
    let world = &s[6..11];
        
    let word = first_word(&s);
    // s.clear(); // wordに不変借用されているため、ここでエラーとなる。
    println!("{}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn dangle() -> &String { // Stringへの参照を返却する
fn dangle() -> String {
    let s = String::from("hello");
    // &s // String s への参照を返そうとするが、sはスコープを抜けたら終了するためエラーとなる。
    s // sを直接返すのであれば問題ない。
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_intager: i32) {
    println!("{}", some_intager);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
