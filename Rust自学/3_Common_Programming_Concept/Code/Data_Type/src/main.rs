fn main() {
    // tuple
    let tup: (u128, u128, u128) = (100, 200, 300);

    let (x, y, z) = tup;

    println!("The value of y is {}", y);

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    println!("The value of first is {}", first);

    // array
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];   // == let a = [3, 3, 3, 3, 3]

    let third = a[2];
    println!("the third value of a is {}", third);
}