// functions can have the same names, e.g.

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz’s impl of Bar"); }
}

let b = Baz;

// but you can't call b.f() as there are multiple functions named thus

// but you can do:

Foo::f(&b);
Bar::f(&b);

// ANGLE BRACKET FORM

// expanded form of Trait::method(args);

<Type as Trait>::method(args);

// <>::  syntax is means of providing a type hint

// it means we want Trait's version of method called here
//  the as Trait part is optional if it's not ambiguous

trait Foo {
    fn clone(&self);
}

#[derive(Clone)]
struct Bar;

impl Foo for Bar {
    fn clone(&self) {
        println!("Making a clone of Bar");

        <Bar as Clone>::clone(self);
    }
}

//  this will call Clone's trait of clone() rather than Foo's