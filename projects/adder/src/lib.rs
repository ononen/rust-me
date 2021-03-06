#[derive(Debug)]

pub struct Rectangle {
    length: u32,
    width: u32,
}

pub struct Guess {
    value: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other:&Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        } 

        Guess {
            value: value, 
        }
    }
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7};
        let smaller = Rectangle { length: 5, width: 1};
        
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle { length: 8, width: 7};
        let smaller = Rectangle { length: 5, width: 1};
        
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("carol");
        assert!(result.contains("carol"));
    }
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
