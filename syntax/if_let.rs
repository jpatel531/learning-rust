/*

if we have some sort of Option<T>,
and we want to call a function on it if it's
Some<T>s, but do nothing if it's None.

*/

match option {
  Some(x) => {foo(x)},
  None => {},
}

// we don't have to use match. we could use if:

if option.is_some(){
  let x = option.unwrap();
  foo(x);
}

// but neither are particularly appealing.

// we can use if let:

if let Some(x) = option { 
  foo(x) // if pattern matches
} else {
  bar();
}

//  WHILE LET

// gnarly way

loop {
  match option {
    Some(x) => println!("{}", x),
    _ => break,
  }
}

// rad way

while let Some(x) = option {
  println!("{}", x);
}
