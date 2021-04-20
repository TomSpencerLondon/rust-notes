#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod add_two_tests {
    use super::add_two;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Value must be less than or equal to 100, got {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod guess_tests {
    use super::Guess;

    #[test]
    #[should_panic(expected = "Value must be less than or equal to 100")]
    fn greater_than_100() { Guess::new(200); }
}