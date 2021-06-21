mod drop;
mod message_passing;
mod multiple_messages;
mod cloned_transmitter;

// use std::io;
use std::mem::drop;
// use std::cmp::Ordering;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {

    // // 1. Drop
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    //
    // println!("CustomSmartPointers created.");

    // 2. Drop values early
    // a) Don't do this:
    // let c = CustomSmartPointer {
    //     data: String::from("Some data"),
    // };
    // println!("CustomSmartPointer created.");
    // c.drop();
    // println!("CustomSmartPointer dropped before the end of main.");

    // b) Do this
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    drop(c);
    println!("CustomSmartPointer dropped before end of main");

}
