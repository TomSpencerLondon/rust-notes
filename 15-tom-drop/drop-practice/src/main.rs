mod drop;

// use std::io;
// use std::mem::drop;
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
    // let c = CustomSmartPointer {
    //     data: String::from("some data"),
    // };
    //
    // drop(c);
    // println!("CustomSmartPointer dropped before end of main");


    //3.  println!("Guess the number!");
    // println!("Please input your guess.");
    //
    // let mut guess = String::new();
    //
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    //
    // println!("You guessed: {}", guess);

    // 4. Guess Game

    // let mut secret_number = 15;
    //
    // loop {
    //     println!("Please input your guess.");
    //     let mut guess = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //
    //     println!("You guessed: {}", guess);
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

}
