struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
  fn area(&self) -> f64 {

    // first param can be self, &self, &mut self
    // use &self by default as you should
    // prefer borrowing over taking ownership
    // and immutable references over mutable ones

    std::f64::consts::PI * (self.radius * self.radius)
  }
}

fn main() {
  let c = Circle{x:0.0, y:0.0, radius: 20.0};
  println!("{}", c.area());
}

//This will print 12.566371.

// CHAINING

 // as above
impl Circle {
  /// as above

  fn grow(&self, increment:f64) -> Circle {
    Circle{x: self.x, y:self.y, radius: self.radius + increment}
  }
}

fn main() {
  // as bove

  let d = c.grow(2.0).area();
  println!("{}", d);
}

// ASSOCIATED FUNCTIONS. functions that do not take a self param

impl Circle {
  fn new(x: f64, y:f64, radius:f64) -> Circle {
    Circle {
      x: x, 
      y: y,
      radius: radius,
    }
  }
}

fn main() {
  let c = Circle::new(0.0,0.0,2.0);

  // assoc functions are with :: syntax
}

//  BUILDER PATTERN

// if we want our users to be able to create circles
// but we will allow them to only set the properties
// they care about. otherwise x, y will be 0 and radius 1
// rust doesn't have method overloading, named arguments
// or variable arguments.
// so you can use the builder pattern

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct CircleBuilder {
  x: f64,
  y: f64,
  radius: f64,
}

impl CircleBuilder {
  fn new() -> CircleBuilder {
    CircleBuilder {x: 0.0, y:0.0, radius:1.0}
  }
  // note &mut sellf
  fn x(&mut self, coordinate:f64) -> &mut CircleBuilder {
    self.x = coordinate;
    self
  }

  fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
    self.y = coordinate;
    self
  }

  fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
    self.radius = radius;
    self
  }

  fn finalize(&self) -> Circle { // finalzie the builder and create the circle
    Circle {x:self.x, y:self.y, radius:self.radius}
  }

}

fn main() {
  let c = CircleBuilder::new()
            .x(1.0)
            .y(2.0)
            .radius(2.0)
            .finalize();

  println!("area: {}", c.area());
  println!("x: {}", c.x);
  println!("y: {}", c.y);
}






















