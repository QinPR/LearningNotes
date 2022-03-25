#[derive(Debug)]
enum UsState {
    Alabama, 
    Alaska,
    // etc
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        Some(i) => Some(i+1),
        _ => None,
    }
}

fn main() {
    let p = Coin::Quarter(UsState::Alaska);
    dbg!(value_in_cents(p));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll{
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => ()
    }
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("lucky penney!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        },
    }
}

fn add_fancy_hat(){}

fn remove_fancy_hat(){}

