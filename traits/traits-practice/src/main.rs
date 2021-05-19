
trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

fn main() {
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();

    println!("I take a bit: {:?}", carrot);
}

#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}