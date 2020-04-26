extern crate communicator;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Greeen,
}

use a::series::of::nested_modules;
// use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    a::series::of::nested_modules();
    nested_modules(); // useキーワードで指定しているため、上と同義

    let red = Red;
    let yellow = Yellow;
    // let green = TrafficLight::Greeen; // Greenは指定していないためフル
    let green = Greeen;
}
