/* Update syntax:

if you want to update a field of a struct,
but keep the others
*/

struct Point3d {
  x: i32,
  y: i32,
  z: i32,
}

let mut point = Point3d{ x: 0, y:0, z:0};

point = Point3d{y:1, ..point};

// this updates the y field value.

// ALSO: Rust doesn't have field-level mutability. The binding to the struct
// must be mutable or not.

// TUPLE STRUCTS:

struct Color(i32, i32, i32);

let black = Color(0,0,0);

/*
There is one case when a tuple struct is very useful, though, 
and that’s a tuple struct with only one element. We call this the ‘newtype’ pattern, 
because it allows you to create a new type, distinct 
from that of its contained value and expressing its
own semantic meaning:
*/

struct Inches(i32);

let length = Inches(10);

let Inches(integer_length) = length;

//  integer_length = 10

// UNIT-LIKE STRUCTS

// struct with no members at all

struct Electron;









