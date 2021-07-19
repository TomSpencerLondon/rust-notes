extern crate traits;
use traits::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a Selectbox
    }
}

// error[E0277]: the trait bound `Box<SelectBox>: Draw` is not satisfied
//   --> src/main.rs:17:18
//    |
// 17 |     let screen = Screen {
//    |                  ^^^^^^ the trait `Draw` is not implemented for `Box<SelectBox>`
//    |
//    = note: required by `Screen`

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
        ]
    };

    screen.run();
}
