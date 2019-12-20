use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    let data = Arc::new(Mutex::new(0));

    // `tx` is the "transmitter" or "sender"
    // `rx` is the "receiver"
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(*data).unwrap();
        });
    }

    for i in 0..10 {
        rx.recv().unwrap();
        println!("{}",i);
    }
}
