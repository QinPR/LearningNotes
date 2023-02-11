#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn phen(&self) -> u32{
        2 * (self.width + self.height)
    }
}

impl Rectangle{
    fn can_hold(&self, another_rect: &Rectangle) -> bool{
        return self.width > another_rect.width && self.height > another_rect.height;
    }
    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of rect1 is {} square pixels",
        rect1.area()
    );

    println!(
        "The phen of rect1 is {} pixels",
        rect1.phen()
    );

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    dbg!(sq);
}

