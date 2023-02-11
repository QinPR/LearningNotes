struct Point<T>{
    x: T,
    y: T,
}

#[derive(Debug)]
struct weird_point<X1, Y1>{
    x: X1, 
    y: Y1,
}

impl<X1, Y1> weird_point<X1, Y1>{
    fn mixup<X2, Y2>(self, other: weird_point<X2, Y2>) -> weird_point<X1, Y2>{
        weird_point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T>{
    Some(T),
    None,
}
enum Result<T, E>{
    Some(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let integer = Point{x: 5, y: 10};
    let charac = Point{x: 'a', y: 'b'};

    let int_p = weird_point{x: 5, y: 1};
    let char_p = weird_point{x: 'a', y: 'b'};

    let stange = int_p.mixup(char_p);

    println!("the mix point is {:?}", stange);
}

// fn largest(list: &[i32])-> i32 {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }

fn largest<T: PartialOrd + Copy>(list: &[T])-> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest{
            largest = item;
        }
    }
    largest
}

