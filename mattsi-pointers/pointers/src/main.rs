fn main() {
    let b = Box::new(String::from("from"));

    let c = &*b;

    print!("{} {}", b, c);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, *y);
}


enum List {
    Cons(i32, Box<List>),
    Nil,
}