pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(origin: usize) -> usize {
    origin + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello!, {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(new_number: i32) -> Guess{
        if new_number < 0{
            panic!("The number is smaller than 0.");
        }else if new_number > 100{
            panic!("The number is larger than 100.");
        }

        Guess {value: new_number}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn test_greeting_function() {
        let name = "David";
        let result = greeting(&name);
        assert!(
            result.contains(&name),
            "Greeting was not containing name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "is larger than 100")]
    fn test_init_guess() {
        Guess::new(200);
    }

    #[test]
    fn it_works_without_panic() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        }else{
            Err(String::from("基本的数学规律不存在了。"))
        }
    }
}
