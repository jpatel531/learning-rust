/*

two main types of string: &str and String

&str => string slices. statically allocated - exsits for the entire runtime
  are called like so:
*/

let string = "Hello there.";

/*
the `string` binding is a reference to this
statically allocated string.
string slices have a fixed size, and are immutable

A `String` on the other hand is heap-allocated.
Growable and guaranteed to be UTF-8.
`String`s are commonly created by converting string slice
using to_string method
*/

let mut s = "Hello".to_string();
println!("{}", s);

s.push_str(", world");
println!("{}", s);

// Strings will coerce to &str with an &;

fn takes_slice(slice: &str){
  println!("Got {}", slice);
}

fn main() {
  let s = "Hello".to_string();
  takes_slice(&s);
}

// viewing String as a &str is cheap, but converting 
// the &str to String involves allocating memory.
// Don't do it unless you have to.

//  INDEXING

// can't use [] operator
// best way is:

let hachiko = "忠犬ハチ公";
let dog = hachiko.chars().nth(1);

// CONCATENATION

// If you have a String, you can concatenate a &str to the end of it:

let hello = "Hello ".to_string();
let world = "world!";

let hello_world = hello + world;

// but if you have two Strings, you need an &:

let hello = "Hello ".to_string();
let world = "world".to_string();

let hello_world = hello + &world;

// &String can automatically coerce to a &str.
// this feature is called Deref coercions


