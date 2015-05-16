// anonymous functions

let plus_one = |x:i32| x + 1;

assert_eq!(2, plus_one(1));

// can have multiline closures

let plus_two = |x| {
  let mut result:i32 = x;
  result += 1;
  result+= 1;
  result;
}

assert_eq!(4, plus_two(2));

// optionally we can choose not to annotate the types
// of arguments to a closure

//  CLOSURES AND THEIR ENVIRONMENT

// closures are called such because they close over their ENVIRONMENT

let num = 5;
let plus_num = |x:i32| x + num;
assert_eq(10, plus_num(5));

//  the closure borrows the `num` binding

/*
this would fuck up:

let mut num = 5;
let plus_num = |x: i32| x + num; // because this borrows num

let y = &mut num; // now you can't get it back until it runs out of scope

this would fix it:
*/

let mut num = 5;
{
  let plus_num = |x:i32| x + num;
}

let y = &mut num;

//  MOVE CLOSURES

// we can force our closure to take ownership of its environment with the `move` keyword

let num = 5;

let owns_num = move |x:i32| x + num;

//  ------
// difference between :

let mut num = 5;

{ 
    let mut add_num = |x: i32| num += x;

    add_num(5);
}

assert_eq!(10, num);

// and :

let mut num = 5;

{ 
    let mut add_num = move |x: i32| num += x;

    add_num(5);
}

assert_eq!(5, num);

// is that in the second, it takes ownership of the copy. in the first
// it takes a mutable borrow on num

// move closures gives a closure its own stack frame
//  without move a closure may be tied to the stackframe that created it
//  move closures are self contained
// . This means that you cannot generally return a n
// non-move closure from a function, for example.

// TAKING CLOSURES AS ARGUMENTS

fn call_With_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32{
  some_closure(1)
}

let answer = call_With_one(|x| x +2);
assert_eq!(3, answer);

/*
call with one takes one param, of type F, and returns i32

   where F : Fn(i32) -> i32 {

Because Fn is a trait, we can bound our generic with it. 
In this case, our closure takes an i32 as an argument and
returns an i32, so the generic bound we use is as above.

if we want dynamic dispatch, we use this:
*/

fn call_With_one(some_closure: &Fn(i32)-> i32)-> i32 {
  some_closure(1)
}

let answer = call_With_one(&|x| x + 2);

assert_eq!(3, answer);

// Now we take a trait object, a &Fn, and we have to make a reference
// to our closure when we pass it to call_with_one, so we use &||


// RETURNING CLOSURES

fn factory() -> Box<Fn(i32) -> i32> {
  let num = 5;
  Box::new(move |x| x + num)
}

let f = factory();

let answer = f(1);
assert_eq(6, answer);

// By making the inner closure a move Fn, we create a new stack frame for our closure. 
// By Boxing it up, we’ve given it a known size, and allowing it to escape our stack frame.

































