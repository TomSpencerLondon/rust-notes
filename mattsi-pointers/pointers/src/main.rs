use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


fn main() {
    let b = Box::new(String::from("from"));

    let c = &*b;

    println!("{} {}", b, c);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
