//  growable array

let v = vec![1,2,3,4,5]; // v: Vec<i32>

let v = vec![0;10] // creates ten zeros

// use index operator to get value

println!("the third element of v is {}", v[2]);

/*
once you have a vector, you can itnerate
through its elements with for:
*/

for i in &v {
  println!("A reference to {}", i);
}

for i in &mut v {
  println!("A mutable reference to {}", i);
}

for i in v {
  println!("Take ownership of the vector and its element {}", i);
}