fn main() {
    // if let式
    // if let式だとコンパイラが網羅性を確認してくれない。
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let条件分岐
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // 条件に合致している間繰り返す。（JavaとかのWhile文と同じだね。
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for ループ
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        // yの値はxの中身の値に束縛される。先に宣言されたyとは別物。
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        // 1から5まで
        1...5 => println!("one through five"),
        // それ以外
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::CangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("quit");
        }
        Message::Move { x, y } => {
            println!("move x: {}, y: {}", x, y);
        }
        Message::Write(text) => println!("Text: {}", text),
        Message::CangeColor(r, g, b) => {
            println!("change color to red {}, greeen {}, blue {}", r, g, b)
        }
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    // マッチガード
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // マッチガードその２
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // @束縛
    let msg = Msg::Hello { id: 5 };
    match msg {
        Msg::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Msg::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        Msg::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }

}

enum Msg {
    Hello { id: i32 }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    CangeColor(i32, i32, i32),
}
