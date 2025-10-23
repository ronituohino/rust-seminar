---
marp: true
theme: rose-pine-moon
paginate: true
footer: Roni Tuohino | rt@ronituohino.com | 20.11.2025
title: Async Rust
author: Roni Tuohino
---

# Async Rust

---

## Terminology

- Serial, Sequential: Finish A, then finish B, then finish C
- Concurrent: Do A, B, and C **alternately**
- Parallel: Do A, B, and C **simultaneously**

- Synchronous, Blocking: A must wait for B to finish  
  <small>(B = blocking, inherently serial)</small>

- Asynchronous, Non-blocking: A can continue while B executes  
  <small>(B = non-blocking, potential for parallelism)</small>

---

## Generally

The idea of asynchronous code is to allow things to run in the background while
doing other stuff (Concurrency). Extremely useful in languges where you don't
want to get into Threads or in environemnts that don't support Parallel
execution.

The idea of `async/await` and `Task/Promise` semantics is to have good control
over asynchronous operations while keeping code easy to read.

---

## But Rust is built different

Compared to other languages, Rust makes it very easy to work with Threads.

---

## The Rust lifetime model with Threads and Async

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
```

---

## `'static` as reference lifetime

Indicates that the data pointed to by the reference lives for the remaining
lifetime of the running program, though it can still be coerced to a shorter
lifetime.

---

## `'static` as trait bound

Mandates that the type does not contain any non-static references. This means
the receiver can hold on to the type indefinitely without it becoming invalid
until they decide to drop it.

---

## `'static` in async

Async tasks often require `'static` data, to guarantee that the data lives long
enough or to ensure ownership is transferred to the task, because tasks in async
runtimes may outlive the scope they were created in.

-> Borrowed data must live as long as the task

-> Hard (or in practice impossible) to use references in async code

-> In synchronous Rust borrowing data across function calls is common

-> Async Rust is fundamentally different

---

"The Original Sin of Rust async programming is making it multi-threaded by
default. If premature optimization is the root of all evil, this is the mother
of all premature optimizations, and it curses all your code with the unholy
`Send + 'static`, or worse yet `Send + Sync + 'static`, which just kills all the
joy of actually writing Rust."

<small>Maciej Hirsz</small>

---

## Threads

If the work is very parallelizable, such as processing a bunch of data where
each part can be processed separately, threads are a better choice.

Requires an OS that supports threads.  
=> OS managed

## Async

If the work is very concurrent, such as handling messages from a bunch of
different sources that may come in at different intervals or different rates,
async **might be** a better choice.

Requires an _async runtime_ (_executor_), which is not built-in to Rust  
=> Software managed

---

## Threads

---

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..3 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

->

hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

---

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("moved value {:?}", v);
        for i in 1..3 {
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

->

moved value [1, 2, 3]
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

---

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += i;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

->

Result: 45
```

---

### Send

Any type that implements `Send` can transfer ownership between threads.

e.g. `Rc<T>` does not implement `Send`, but `Arc<T>` does.

### Sync

Any type that implements `Sync` can be referenced from multiple threads.

e.g. `RefCell<T>` and `Cell<T>` do not implement `Sync`, but `Mutex<T>` does.

---

Types composed entirely of other types that implement the `Send` and `Sync`
traits also automatically implement `Send` and `Sync`.

---

"Don't communicate by sharing memory; share memory by communicating"  
<small>(R.Pike)</small>

---

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
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

->

Got: 45
```

---

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
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

->

Not received yet, doing other work...
Not received yet, doing other work...
Not received yet, doing other work...
Not received yet, doing other work...
Got: hi
```

---

My take from ;

You can do a lot of things with Threads and Channels. Avoid

---

## Async

---

Hello

---

`async`

`await`

`Future`

Polling
