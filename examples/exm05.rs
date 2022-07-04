use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let var = String::from("Hi");
        tx.send(var).unwrap();
    });

    let receiver = rx.recv().unwrap();

    println!("Got {}!", receiver);
}