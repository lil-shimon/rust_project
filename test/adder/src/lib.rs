mod guess;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_number(a: i32) -> i32 {
    return a + 2;
}

pub fn greeting (name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test failed");
    }

    #[test]
    fn larget_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 2,
            height: 3,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_add_two () {
        // (expected, actual)
        // assert_eq => when expected == actual, true
        assert_eq!(4, add_number(2));
        // assert_ne => when expected != actual, true
        assert_ne!(100, add_number(2));
    }

    #[test]
    fn greeting_contains_name () {
        let result = greeting("shimon");
        assert!(result.contains("shimon"), "Greeting did not contain name, Value was {}", result);
    }

    #[test]
    #[should_panic(expected="Guess value must be smaller than one")]
    fn guess_greater_than_100 () {
        guess::Guess::new(-1);
    }
}

