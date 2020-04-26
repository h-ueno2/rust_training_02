use std::collections::HashMap;

fn main() {
    vector_example();

    string_example();

    hashmap_example();
}

fn hashmap_example() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // HashMapからの取得
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score :{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 所有権について
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_value); // 所有権が移動しているためエラーとなる


    // キーに値がなかった時のみ挿入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 値に基づいて更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
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
