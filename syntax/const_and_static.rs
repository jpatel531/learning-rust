const N: i32 = 5;

//  consts are constant and live
// for the entire lifetime of a program
// no fixed address in memory
// must annotate type

static N: i32 = 5;

// static provides a kind of 'global variable' facility.
// fixed location in memory
// static lifetime - entire lifetime of programs
// must annotate

// MUTABILITY

static mut N:i32 = 5;

//  can cause memory unsafety / data race
//  accessing and mutatic a statuic mut must 
//  be done in an unsafe block

unsafe {
  N += 1;
  println!("N: {}", N);
}