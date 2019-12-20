#![allow(unused)]
fn main() {
use std::cell::RefCell;
use std::thread;

thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

FOO.with(|f| {
    assert_eq!(*f.borrow(), 1);
    println!("change from 1 to 2");
    *f.borrow_mut() = 2;
});

// each thread starts out with the initial value of 1
let t = thread::spawn(move|| {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        println!("change from 1 to 3");
        *f.borrow_mut() = 3;
    });
});

// wait for the thread to complete and bail out on panic
t.join().unwrap();

// we retain our original value of 2 despite the child thread
FOO.with(|f| {
    assert_eq!(*f.borrow(), 2);
    println!("2");
});
}
