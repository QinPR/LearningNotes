use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let f = match f{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem open the file {:?}", other_error)
            },
        }
    };

    let f = File::open("hello_b.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_b.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        }else {
            panic!("Problem opening the file {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::fs;
use std::io;

fn short_cut() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn withOPtion(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}