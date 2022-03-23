struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User{
        email: String::from("119010249@link.cuhk.edu.cn"),
        username: String::from("qpr"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("1905873179@qq.com");

    let user2 = User{
        email: String::from("11111111@qq.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
