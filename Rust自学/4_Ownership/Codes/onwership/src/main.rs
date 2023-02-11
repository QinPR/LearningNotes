fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
    {
        let s1 = String::from("Hi");
        let s2 = s1;
        println!("{}", s2);

        let i1 = 5;
        let i2 = i1;
        println!("{}", i1);
    }

    let s1 = gives_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2);     since the ownership of s2 is given to s3, this line will give error

    let (s4, len) = calculate_length(s3);
    // println!("{}", s3);     same error here
    
    println!("The len of {} is {}", s4, len);
}

fn gives_ownership() -> String{
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

fn calculate_length(s:String) -> (String, usize){
    let length = s.len();

    (s, length)
}