 // Functions also have a type! They look like this:

 fn foo(x: i32) -> i32 { x }

 let x: fn(i32) -> i32 = foo;

 //In this case, x is a ‘function pointer’ to a function that takes an i32 and returns an i32.