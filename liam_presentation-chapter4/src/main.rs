fn main() {
    println!("Hello, world!");
    rust_is_smart_and_that_can_be_confusing();
    let mut pawn = Pawn {location: (0, 0) };
    pawn.forward_one();

    let mut queen = pawn.promote();
    queen.diagonal();
}

fn rust_is_smart_and_that_can_be_confusing() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}, and", r1, r2);

    s.push_str("hello");
    let r3 = &mut s;
    r3.push_str("hello");
}

struct Pawn {
    location: (u8, u8),
}

impl Pawn {
    fn forward_one(&mut self) { self.location.1 += 1; }
    fn promote(self) -> Queen {
        Queen { location: self.location }
    }
}

struct Queen {
    location: (u8, u8),
}

impl Queen {
    fn diagonal(&mut self) { self.location.1 += 1; }
}