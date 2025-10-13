use std::thread;
use std::time::Duration;

pub fn threads() {
    let v = vec![1, 2, 3];

    thread::spawn(move || {
        println!("Here's a vector: {v:?}");
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn thread_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

use std::sync::mpsc;

pub fn channels() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel::<std::string::String>();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // blocking i/o
    //let received = rx.recv().unwrap();
    let received_nb = rx.try_recv().unwrap();

    //println!("Got: {received}");
    println!("Got nb: {received_nb}");
}
