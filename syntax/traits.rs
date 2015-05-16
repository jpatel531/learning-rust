// like interfaces/protocols

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// HERE IS OUR TRAIT:

trait HasArea {
    fn area(&self) -> f64; // just give a type signature
}

// `for`
impl HasArea for Circle {
  fn area(&self) -> f64 {
    std::f64::constants::PI * (self.radius * self.radius)
  }
}

// we can use traits to constrain our generics
              // type that implements HasArea trait
fn print_area<T: HasArea>(shape:T) {
  println!("This shape has an area of {}", shape.area());
}

// remember that: if the trait isn’t defined in your scope, it doesn’t apply

// MULTIPLE TRAIT BOUNDS

//  if you need more than one bound, use +

fn foo<T: Clone + Debug>(x: T) {}

// `where` can help clean up multiple trait bounds

fn bar<T,K>(x:T, y:K) 
    where T: Clone, K: Clone + Debug {}

//  `where ` is more powerful than the simpler syntax, eg..:

trait ConvertTo<Output> {
  fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
  fn convert(&self) -> i64 {*self as i64}
}

// can be called with T==i32

fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
  x.convert();
}

fn inverse<T>() -> T 
    where i32: ConvertTo<T> {

  1i32.convert()
}

/*
This shows off the additional feature of where clauses: 
they allow 
bounds where the left-hand side is an
arbitrary type (i32 in this case), not just a plain
type parameter (like T).
*/


//  DEFAULT METHODS

trait Foo {
  fn bar(&self);
  fn baz(&self) {println!("We called baz.");}
}

// implementers of Foo need to implement bar but not baz, cause they'll
// get this default behaviour
// 
// They can override this default if they choose by implementing it
// themselves

// INHERITANCE

trait Foo {
  fn foo (&self);
}

trait FooBar: Foo {
  fn foobar(&self);
}

// implementors of foobar have to implement foo

struct Baz;

impl Foo for Baz {
  fn foo(&self) {println!("foo");}
}

impl FooBar for Baz {
  fn foobar(&self) {println!("foobar");}
}












