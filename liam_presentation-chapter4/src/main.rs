fn main() {
    println!("Hello, world!");
    rust_is_smart_and_that_can_be_confusing();
    let mut pawn = Pawn {location: (0, 0) };
    pawn.forward_one();

    let mut queen = pawn.promote();
    queen.diagonal();

    println!("{}", 100_i32.rem_euclid(60).to_string());
    println!("{}", (17_i32 + 3_i32.div_euclid(60)).rem_euclid(24).to_string());
    println!("{}", 70_i32.div_euclid(60).to_string());
}

fn rust_is_smart_and_that_can_be_confusing() {
    let mut s = String::from("single-thread");
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}, and", r1, r2);

    s.push_str("single-thread");
    let r3 = &mut s;
    r3.push_str("single-thread");
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