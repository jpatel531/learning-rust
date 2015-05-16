trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

fn do_something<T: Foo>(x:T){
  x.method();
}

fn main() {
  let x = 5u8;
  let y = "Hello".to_string();

  do_something(x);
  do_something(y);
}

/*
calling do_something uses static dispatch. 
Rust will create a special version of do_something for both u8 and String
then replace the call sites with those versions.
*/

//  DYNAMIC DISPATCH

// uses 'trait objects'
/*

Trait objects like &Foo or Box<Foo> are normal values that store a
value of any type that implements the given trait, where precise type
can be known at runtime.

Trait object can be obtained from a pointer to a concrete type
that implements the trait by casting it:

  &x as &Foo

or coercing it:

  &x as an argument to a function that takes &Foo

These trait object coercions and casts also work for 
pointers like &mut T to &mut Foo and Box<T> to Box<Foo>, 
but that’s all at the moment. Coercions and casts are 
identical.
*/

// dynamic dispatch with casting:

fn do_something(x: &Foo) {
  x.method();
}

fn main() {
  let x = 5u8;
  do_something(&x as &Foo);

// coercing

fn do_something(x: &Foo){
  x.method();
}

fn main() {
  let x = "Hello".to_string();
  do_something(&x);
}

