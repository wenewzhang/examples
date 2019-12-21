#![allow(unused)]
fn main() {
// use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

thread_local!(static FOO: Arc<Mutex<u32>> = Arc::new(Mutex::new(1)));

FOO.with(|f| {
    // assert_eq!(*f.borrow(), 1);
    println!("change from 1 to 2");
    let mut num = *f.lock().unwrap();
    num  += 2;
});

// each thread starts out with the initial value of 1
let t = thread::spawn(move|| {
    FOO.with(|f| {
        // assert_eq!(*f.borrow(), 1);
        println!("change from 1 to 3");
        let mut num = *f.lock().unwrap();
        num  += 2;
    });
});

// wait for the thread to complete and bail out on panic
t.join().unwrap();

// we retain our original value of 2 despite the child thread
FOO.with(|f| {
    // assert_eq!(*f.borrow(), 2);
    let num = *f.lock().unwrap();
    println!("{}",num);
});
}
