// ordered list of fixed size:

let x = (1, "hello");

// type annotated version:

let x: (i32,&str) = (1, "hello");

// you can assign one tuple to another, if same types and length

let mut x = (1,2);
let y = (2,3);

x = y;

// you can access the fields in the tuple through destructuring let:

let (x,y,z) = (1,2,3);

println!("x is {}", x);

// indexing:

let tuple = (1,2,3);

let x = tuple.0;
let y = tuple.1;
let z = tuple.2;

println!("x is {}", x);