fn main() {
    let mut s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;
    let r3 = &mut s1;
    println!("{}, {}, {}", r1, r2, r3);    // erorr will occur here

    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);

    change(&mut s1);

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn dangle() -> &String{
    let s = String::from("hello");

    &s // error will occur here, because the value of s will be out of scope before the reference, and cause a dangling reference.
}