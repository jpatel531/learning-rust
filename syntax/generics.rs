/*

example of Option<T> in Rust's stdlib, which is generic

*/

enum Option<T> {
  Some(T),
  None,
}

// <T> indicates it's a generic type. 
// Inside the declaration of our enum, wherever we see a T,
 // we substitute that type for the same type used in the generic. 

let x : Option<i32> = Some(5);

// Rust is happy because 5 is an i32

// if  :

let x:Option<f64> = Some(5);

// unhappy, because 5 isn't an f64
// they have to match up

// don©enerics don't only have to be generic over one type, e.g.

enum Result<T,E> {
  Ok(),
  Err(E),
}

// we can write functions that take generic types with a similar synaax:

fn takes_anything<T>(x:T){
  // do something with x
}

// you can have same generic type:

fn takes_two_of_the_same_things<T>(x: T, y T) {}

//  and with multiple types:

fn takes_two_things<T,U>(x:T, y:u){}

// GENERIC STRUCTS

struct Point<T> {
  x:T,
  y:T,
}

let int_origin = Point { x: 0, y: 0 };
let float_origin = Point { x: 0.0, y: 0.0 };

