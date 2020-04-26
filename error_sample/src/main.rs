use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

fn main() {
    // unwrap()はエラーだったらpanicしてくれる
    // let f = File::open("hello.txt").unwrap(); 

    // expectはエラーだった時のメッセージも選択可能
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a prblem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // // 戻り値
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // 下記は上記と同義になる(?演算子を使用して書き換えした場合)
    // ?演算子を付けると上記で使用しているmatch式と同じ動きをする
    // ただしエラー内容にかかわらずすべて同じ動作となるため、
    // 用途によっては注意。
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    
    // さらに以下のようにも書ける
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

