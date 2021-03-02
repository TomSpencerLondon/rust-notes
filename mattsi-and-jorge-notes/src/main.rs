#[derive(Debug)]
enum Weapon {
    Sword(i32),
    Mace(f32),
    Toy
}

#[derive(Debug)]
enum Result<T,V> {
    Success(T),
    Error(V),
}

fn main() {
    let weapon1 = Weapon::Sword(8);
    println!("{:?}", weapon1);

    println!("{}", hit(&weapon1));

    let result = process_something(10);
    if let Result::Success(10) = result {
        println!("Success {}", 20);
    } else {
        println!("Something");
    }
}

fn process_something(value: i32) -> Result<i32, String> {
    if value == 5 {
        return Result::Error("Oh well".to_string());
    } else {
        return Result::Success(2 * value);
    }
}

fn hit(weapon: &Weapon) -> String {
    match weapon {
        Weapon::Sword(damage) => format!("The sword does {}", damage),
        Weapon::Mace(damage) => format!("The mace does {}", damage),
        Weapon::Toy => "The toy does no damage".to_string(),
    }
}
