// A type that represents data that could be of several possible variants:

enum Message {
  Quit,
  ChangeColor(i32,i32,i32), // variant declaration similar to declaring structs
  Move {x: i32, y: i32},
  Write(String),
}

// value of the enum can match any of the variants
/*
enum is sometimes called a sum type
the set of possibile values of the enum is the sum
of the sets of possible values for each variant
*/

let x: Message = Message::Move {x: 3, y:4};

enum BoardGameTurn {
  Move{ squares: i32},
  Pass,
}

//  Both variants called Move, but are scoped to their
// specific enum 

// MATCH 

// in lieu of a switch-case

let x = 5;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}

/* _ is the default. match has to be exhaustive. 
without _ you'd get an error as it's inexhaustive

match is an expression, so you can do this:
*/

let x = 5;

let number = match x {
    1 => "one",
    2 => "two",
    3 => "three",
    4 => "four",
    5 => "five",
    _ => "something else",
};



//  MATCHING ON ENUMS


// take the Message enum from above

fn quit(){}
fn change_color(r: i32, g: i32, b: i32) {}
fn move_cursor(x: i32, y: i32){}

fn process_message(msg: Message){
  match msg{
    Message::Quit => quit(),
    Message::ChangeColor(r, g, b) => change_color(r,g,b),
    Message::Move{x: x, y: y} => move_cursor(x, y),
    Message::Write(s) => println!("{}", s);
  };
}

// demands a match arm for every variant of the enum
















