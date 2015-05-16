/*
Arrays are of immutable length
*/

let a = [1,2,3];
let mut m = [1,2,3];

/*
Arrays have type [T;N]  --> T is a generic. N is a compile-time constant, the 
length of the array

To initialize a 20-element array, each with value 0:
*/

let a = [0;20] // a: [i32; 20]

a.len() // gets length of array