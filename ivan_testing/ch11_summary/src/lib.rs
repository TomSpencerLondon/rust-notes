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


