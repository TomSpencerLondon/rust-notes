// In computability theory, the halting problem
// is the problem of determining, from a description
// of an arbitrary computer program and an input,
// whether the program will finish running,
// or continue to run forever. Wikipedia

// ref cell muttate

fn main() {
    println!("Hello, world!");
}

pub trait Messenger {
    // takes immutable reference to self
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{

}