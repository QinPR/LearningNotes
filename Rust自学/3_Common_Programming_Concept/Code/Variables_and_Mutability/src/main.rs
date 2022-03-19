fn main() {
    
    // const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // mut & immut
    let mut x = 5;
    println!("The first value of x is {}", x);
    x = 6;
    println!("After changing the value of x, it has {}", x);

    // shadow 1
    let a = 5;
    let a = a + 1;
    {
        let a = a * 2;
        println!("The value of a rn is {}", a);
    }

    println!("The value of a out of scope is {}", a);

    // shadow 2
    let mut letters = "abcde";
    {
        let letters = letters.len();
        println!("The value of letters rn is {}", letters);
    }

    println!("The value of letters out of scope is {}", letters);
}
