// extern crate phrases;

// // use phrases::engish::greetings; // puts greetings into scope
// // use phrases::english::farewells; // as above
// // shortcut:

// use phrases::english::{greetings, farewells};

// fn main() {
//     println!("Hello in English: {}", greetings::hello());
//     println!("Goodbye in English: {}", phrases::english::farewells::goodbye());

//     // println!("Hello in Japanese: {}", phrases::japanese::greetings::hello());
//     // println!("Goodbye in Japanese: {}", phrases::japanese::farewells::goodbye());
// }

extern crate phrases;

use phrases::english::{greetings,farewells};
use phrases::japanese;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());
}