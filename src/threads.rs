use std::thread;
use std::time::Duration;

pub fn threads() {
    // non-blocking
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
    // blocking with join()
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

// complex inter-thread communication with mutexes
use std::sync::{Arc, Mutex};
pub fn inter_thread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// channels offer a nicer alternative
use std::sync::mpsc;
pub fn channels() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel::<std::string::String>();
    // let tx1 = tx.clone(); ... (multiple)

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // blocking i/o
    let rec = rx.recv().unwrap();
    println!("Got: {rec}");

    // for rec_it in rx {
    //     println!("Got {rec_it}")
    // }
}

pub fn async_with_channels() {
    let (tx, rx) = mpsc::channel::<std::string::String>();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    loop {
        let received_nb = rx.try_recv();
        match received_nb {
            Ok(val) => {
                println!("Got: {val}");
                break;
            }
            Err(_) => println!("Not received yet, doing other work..."),
        }
    }
}
