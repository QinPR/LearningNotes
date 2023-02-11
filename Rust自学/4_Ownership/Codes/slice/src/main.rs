fn main() {
    let mut s1 = String::from("Hello, world!");

    let mut s2 = "hi world!";

    // s2.clear();    error here since s2 is a slice which is a immutable &str

    let word = first_word(&s1);

    s1.clear();

   //println!("the first word is {}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    println!("the slice is {}", slice[0]);
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    
    &s[..]
}