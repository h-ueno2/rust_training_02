fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is: {}", spaces);

    let x = 2.0;
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("the tup.1 is: {}", tup.1);

    another_function(5);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y); // 4
    println!("The value of x is: {}", x); // 5

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x)
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
