//

use std::time::Duration;
use std::thread;
use std::intrinsics::likely;

fn main() {
    println!("Hello, world!");

    // let example_closure = |x| x;
    //
    // let s = example_closure(String::from("single-thread"));
    // let n = example_closure(5);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    }else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}