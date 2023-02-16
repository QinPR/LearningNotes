#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    price: u32,
}

fn filter_out_bigger_shoes(shoes: Vec<Shoe>, target_value: u32) -> Vec<Shoe> {
    // I think the filter + closure must be one of the most common used function in rust!!
    shoes.into_iter().filter(|x| x.size <= target_value).collect()   // we call into_iter() instead of iter() to take the ownership of shoes, then we can call filter.
}


fn main() {
    let list = vec![1, 2, 3];

    for number in list.iter() {
        println!("the value is {}", number);
    }

    let mut list_iter = list.iter();   // mut here because every time it calls next(), the iterator should keep track on the 'position'

    println!("the next value is {:?}", list_iter.next());


    let v1 = vec![2, 3, 4];

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(vec![3, 4, 5], v2);


    // filter out all the shoes bigger than some value
    let shoes_list = vec![
        Shoe{
            size: 42,
            price: 100,
        },
        Shoe{
            size: 39,
            price: 150,
        },
        Shoe{
            size: 45,
            price: 50,
        }
    ];
    let remaining_shoes = filter_out_bigger_shoes(shoes_list, 40);
    println!("the ramaining shoes are {:?}", remaining_shoes);
}
