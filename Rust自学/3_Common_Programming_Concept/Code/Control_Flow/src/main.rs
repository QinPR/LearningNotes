fn main() {
    let number :i32 = 3;

    if number < 5{
        println!("The condition is true");
    }
    else if number > 7{
        println!("The condition is greater than 7");
    }
    else{
        println!("The condition is false");
    }

    let word = if 1<2 {5} else {6};
    println!("The word now is {}", word);

    loop_function();

    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2
        }
    };

    println!("the result is {}", result);

    while_loop();

    for_loop();
}

fn loop_function(){
    let mut count = 0;
    'counting_up: loop{
        println!("count = {}", count);
        let mut remaining = 10;

        loop{
            println!("remaining = {}", remaining);
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn while_loop(){
    let mut number = 3;

    while number != 0{
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("the value is {}", element);
    }

    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LISTOFF!!!");
}