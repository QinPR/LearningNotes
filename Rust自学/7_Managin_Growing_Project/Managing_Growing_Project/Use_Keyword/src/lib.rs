use rand::Rng;
use std::collections::HashMap;

// use std::{cmp::Ordering, io};

use std::io::{self, Write};

use std::collections::*;


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


use std::io::Result as IoResult;

#[cfg(test)]
mod tests {

    // use crate::front_of_house::hosting;
    use super::front_of_house::hosting;

    #[test]
    fn it_works() {

        let result = hosting::give_two();
        assert_eq!(result, 2);
    }
}

