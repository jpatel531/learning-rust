/*
Like python slices. A slice of an array
*/

let a = [0,1,2,3,4];

let middle = &a[1..4]; // elements 1,2,3

let complete = &a[..]; // all elements

// Slices have type &[T]