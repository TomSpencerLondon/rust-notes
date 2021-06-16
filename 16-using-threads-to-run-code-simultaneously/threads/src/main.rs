use std::thread;
use std::time::Duration;
mod move_closure;
mod mutex_practice;
mod mutex_threads;

fn main() {
    // Ex 1 - Creating a new thread
    // to print one thing while the main
    // thread prints something else
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // for i in 1..5 {
    //     println!("Hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // Ex 2 Saving a joinHandle from
    // thread::spawn to guarantee the thread is run
    // to completion

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
