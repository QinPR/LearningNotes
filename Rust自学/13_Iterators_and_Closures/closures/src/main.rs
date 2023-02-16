use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum shirt_color {
    red,
    blue
}

struct inventory {
    shirt: Vec<shirt_color>,
}

impl inventory {
    fn give_away(&self, user_preference: Option<shirt_color>) -> shirt_color{
        user_preference.unwrap_or_else(|| self.the_most_one_in_inventory())
    }

    fn the_most_one_in_inventory(&self) -> shirt_color {
        let mut red_num = 0;
        let mut blue_num = 0;

        for color in &self.shirt {
            match color {
                shirt_color::red => red_num += 1,
                shirt_color::blue => blue_num += 1,
            }
        }
        
        if red_num > blue_num {
            shirt_color::red
        } else {
            shirt_color::blue
        }
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let shirt_company = inventory {
        shirt: vec![shirt_color::red, shirt_color::blue],
    };

    let user1_preference = Some(shirt_color::red);

    let user1_color = shirt_company.give_away(user1_preference);

    let user2_preference = None;

    let user2_color = shirt_company.give_away(user2_preference);

    println!("user 1 got the color {:?}", user1_color);
    println!("user 2 got the color {:?}", user2_color);

    let expensice_cost = |num: i32| -> i32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let one_expensive = expensice_cost(2);

    println!("the value of one_expensive is {}", one_expensive);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();


    let mut list = vec![
        Rectangle{width: 10, height: 30},
        Rectangle{width: 15, height: 20},
        Rectangle{width: 60, height: 100},
    ];

    let mut num_of_sort_operation = 0;
    list.sort_by_key(|r| {
        num_of_sort_operation += 1;
        r.width
    });

    println!("The list becomes {:?} after {} times of sorting.", list, num_of_sort_operation);
}
