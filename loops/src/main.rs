fn main() {
    // while を使ったループ
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!");

    // for を使ったループ
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    
    // rev()メソッドで逆順
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}
