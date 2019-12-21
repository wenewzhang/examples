#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::*;
use std::sync::Mutex;

lazy_static! {
    static ref G: Mutex<(Sender<i32>, Option<Receiver<i32>>)> = {
        let (tx, rx) = channel();
        Mutex::new((tx, Some(rx)))
    };
}

fn main() {
    let rx = G.lock().unwrap().1.take().unwrap();
    // move rx to background thread
}

fn log() {
    thread_local! {
        static S: Sender<i32> = G.lock().unwrap().0.clone();
    }
    S.with(|sender| sender.send(5));
}
