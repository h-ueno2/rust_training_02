fn main() {
    vector_example();

    string_example();
}

fn vector_example() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);

    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);


    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn string_example() {
    // 空のStringを作成
    let mut s = String::new();
    
    // 以下は等価
    let s = String::from("initial contents");
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    // 文字列の追加
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l'); // pushは1文字

    // 文字列の連結
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
}
