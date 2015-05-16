// patterns already covered: variable bindings and match

// MULTIPLE PATTERNS:

// can match multiple with |

let x = 1;

match x {
  1 | 2 => println!("one or two"),
  3 => println!("three"),
  _ => println!("andything")
}

// prints "one or two"

// can match a range of values with ... :

let x = 1;

match x {
  1 ... 5 => println!("one through five"),
  _ => println!("anything"),
}

//  can bind values to names with `@`

let x = 1;

match x {
  e @ 1 ... 5 => println!("got a range element {}", e),
  _ println!("something else!"),
}

// prints 'got a range element 1

// if you use @ with |, make sure the name is bound in each part of the pattern:

let x = 5;

match x {
  e @ 1...5 | e @ 8...10 => println!("got a range element {}", e),
  _ => println!("anything else");
}

// IGNORING VARIANTS

enum OptionalInt {
  Value(i32),
  Missing,
}

let x = OptionalInt::Value(5);

match x {
  OptionalInt::Value(..) => println!("Got an int"),
  OptionalInt::Missing => println!("No such luck"),
}

// .. ignores the value an type in the variant

//  GUARDS

// you can introduce match guards with if

let x = OptionalInt::Value(5);

match x {
  OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than 5"),
  OptionalInt::Value(..) => println!("Got an int"),
  OptionalInt::Missing => println!("No such luck"),
}

// prints 'got an int'

// REF AND REF MUT

let x = 5;

match x {
  ref r => println!("Got a reference to {}", r),
}

// prints "got a reference to 5s"
// ref creates a reference for use in the pattern.
// ref must can be used in the same way

let mut x = 5;

match x {
  ref mut mr  => println!("Got a mutable reference to {}", mr),
}

// Destructuring

// you can destructure a struct inside of a pattern

struct Point {
  x: i32,
  y: i32,
}

let origin = Point{x:0,y:0};

match origin {
  Point {x:x, y:y} => println!("({}, {})", x, y),
}

// if we don't care about some of the values, we don't have to give them all names
// we can use '..'

match origin {
  Point {x: x, ..} => println("x is {}", x),
}

// prints 'x is 0'


// you can do this on any member of the struct, not just the first

















