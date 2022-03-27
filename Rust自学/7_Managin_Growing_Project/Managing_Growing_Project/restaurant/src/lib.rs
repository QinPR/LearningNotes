mod front_of_house{
    pub mod hosting{
        fn add_to_waitlist(){}

        fn seat_at_table(){}

        pub fn give_two() -> i32{
            2
        }
    }

    mod serving{
        fn take_over(){}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer() -> String{
            "peaches".to_string()
        }
    }

    pub enum Appetizer{
        soup,
        Salad,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::front_of_house::hosting::give_two();
        assert_eq!(result, 2);
    }

    #[test] 
    fn test_private() {
        let seasonal_fruit = crate::back_of_house::Breakfast::summer();
        assert_eq!(seasonal_fruit, "peaches".to_string());
    }
}

pub fn hello(){
    let result = front_of_house::hosting::give_two();
}
