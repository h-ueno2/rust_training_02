use std::io::Error;
use std::io::Result;
use std::fmt;

pub trait Write {
    // std::io::Resultの型エイリアスを使うことでErrorの記載を省略できる。
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

}

fn main() {

    type kiloemeters = i32;

    let x: i32 = 5;
    let y: kiloemeters = 5;

    println!("x + y = {}", x + y );

    let f: Thunk = Box::new(|| println!("hi"));

    
}

type Thunk = Box<Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    
}

fn bar() -> ! {
    // --snip--
}
