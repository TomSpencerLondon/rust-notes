use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

fn main() {
    // 1. Using Rc - A single-threaded reference-counting pointer.
    // ‘Rc’ stands for ‘Reference Counted’.

    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter.lock().unwrap());

    // 2. Using Arc for mutex threads:
    // A thread-safe reference-counting pointer.
    // ‘Arc’ stands for ‘Atomically Reference Counted’.
    //
    // The type Arc<T> provides shared ownership of a value of type T,
    // allocated in the heap. Invoking clone on Arc produces a new Arc
    // instance, which points to the same allocation on the heap as
    // the source Arc, while increasing a reference count.
    // When the last Arc pointer to a given allocation is destroyed,
    // the value stored in that allocation (often referred to as “inner value”)
    // is also dropped.


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