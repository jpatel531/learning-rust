/* 
Drop is a trait from the stdlib
Provides a way to run some code for 
whenever a value goes out of scope
e.g. */

struct HasDrop;

impl Drop for HasDrop {
  fn drop(&mut self){
    println!("Dropping!");
  }
}

fn main() {
  let x = HasDrop;
}

// last in first out:
// values are dropped in the opposite order
// they are declared:

struct Firework {
  strength: i32,
}

impl Drop for Firework {
  fn drop(&mut self){
    println!("B");
  }
}

fn main() {
  let firecracker = Firework { strength: 1 };
  let tnt = Firework{strength:100}; 
}

// BOOM times 100!!!
// BOOM times 1!!!

// tnt goes off first as it was declared last