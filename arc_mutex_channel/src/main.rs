use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::{thread::sleep, time::Duration};

fn main() {
    let data = Arc::new(Mutex::new(10));

    // `tx` is the "transmitter" or "sender"
    // `rx` is the "receiver"
    let (tx, rx) = mpsc::channel();

    // for _ in 0..10 {
        let (data2, tx2) = (data.clone(), tx.clone());

        thread::spawn(move || {
            loop {
            let mut data = data2.lock().unwrap();
            *data += 1;

            tx2.send(*data).unwrap();
            sleep(Duration::from_millis(10));
        }
        });
    // }
    // let rx2 = rx.clone();

    thread::spawn(move || {
        loop {
        let data3 = rx.recv().unwrap();
        println!("{}",data3);
        sleep(Duration::from_millis(10));
        }
    });
    sleep(Duration::from_secs(10));
    // for i in 0..10 {
    //     rx.recv().unwrap();
    //     println!("{}",i);
    // }
}
