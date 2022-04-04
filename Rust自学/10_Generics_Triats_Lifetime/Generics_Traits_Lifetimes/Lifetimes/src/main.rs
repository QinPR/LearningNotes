struct ImportantExcept<'a> {
    part: &'a str, 
}

impl<'a> ImportantExcept<'a>{
    
}

fn main() {
    let string1 = String::from("abcd");

    let result;
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("the longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept{
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime";

    // {
    //     let x = String::from("I will die soon");

    //     let d: &'static str = x.as_str();
    // }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else{
        y
    }
}