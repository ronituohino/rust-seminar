// channels offer a nicer alternative
use std::sync::mpsc;
use std::thread;

pub fn channels() {
    let (tx, rx) = mpsc::channel::<i32>();
    let mut handles = vec![];

    for i in 1..10 {
        let tx_h = tx.clone();
        let handle = thread::spawn(move || {
            tx_h.send(i).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut total = 0;
    for v in rx.try_iter() {
        total += v;
    }
    println!("Got: {total}");
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
