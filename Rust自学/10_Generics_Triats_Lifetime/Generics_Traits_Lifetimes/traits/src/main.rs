pub trait Summary {
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, {}", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}

fn return_summarizable() -> impl Summary{
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("nothing at all"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let article = NewsArticle{
        headline: String::from("This is a NEWS"),
        location: String::from("shenzhen"),
        author: String::from("qpr"),
        content: String::from("nothing"),
    };
    
    println!("new article available! {}", article.summarize());

    notify(&article);
}
