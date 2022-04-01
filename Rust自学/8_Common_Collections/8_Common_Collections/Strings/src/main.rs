fn main() {

    let mut s = String::new();
    
    let data = "init contents";
    let s = data.to_string();

    let mut s = String::from("init contents");

    s += " Hi!";
    println!("after adding first: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = String::from("!");

    let s = format!("{}{}{}", s1, s2, s3);

    let s1 = String::from("hello");
    // let h = s1[0];

    for c in "你好啊".chars(){
        println!("{}", c);
    }

    for b in "你好啊".bytes(){
        println!("{}", b);
    }
}
