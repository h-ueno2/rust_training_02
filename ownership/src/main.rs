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
