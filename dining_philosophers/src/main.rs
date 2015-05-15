use std::thread;
use std::sync::{Mutex,Arc};

struct Philosopher {
  name: String,
  left: usize, // usize is the type you index vectors with
  right: usize,
}

// for creating associated functions
impl Philosopher {
  fn new(name: &str, left: usize, right: usize) -> Philosopher {
    Philosopher {
      name: name.to_string(),
      left: left,
      right: right,
    }
  }

  fn eat(&self, table: &Table){

    let _left = table.forks[self.left].lock().unwrap(); // lock will block until available
    let _right = table.forks[self.right].lock().unwrap();
    // prefix underscore means we won't be using that value. avoids compiler warnings

    println!("{} is eating.", self.name);

    thread::sleep_ms(1000);

    println!("{} is done eating.", self.name);

    // _left and _right will be released when they go out of scope
  }

}

struct Table {
    forks: Vec<Mutex<()>>, // only one thread has access to the contents at once
} // empty tuple inside the mutex - as we're not going to use it, just hold onto it


fn main() {

    let table = Arc::new(Table {forks: vec![ // arc stands for atomic reference count
        Mutex::new(()),                       // shares table across multiple threads
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 0,1),
        Philosopher::new("Gilles Deleuze", 1,2),
        Philosopher::new("Karl Marx",2,3),
        Philosopher::new("Friedrich Nietzsche",3,4),
        Philosopher::new("Michel Foucault",0,4),
    ];

    // handles lets us control our threads' operations
    // _ is a placeholder

    // iter creates an iterator that lets us take ownership of each 
    // philosopher
    let handles: Vec<_> = philosophers.into_iter().map(|p|{

        let table = table.clone(); // .clone() bumps up the reference count

        thread::spawn(move || { // move indicates that the closure is going to take ownership of the values it's capturing, i.e. `p`
            p.eat(&table);
        })
    }).collect(); // collect() will turn them into a collection of the return values of the thread::spawn calls

    // for p in &philosophers {
    //     p.eat();
    // }
    for h in handles {
        h.join().unwrap(); // join blocks the execution until the thread has completed (makes sure they've finished before program ends)
    }

}
