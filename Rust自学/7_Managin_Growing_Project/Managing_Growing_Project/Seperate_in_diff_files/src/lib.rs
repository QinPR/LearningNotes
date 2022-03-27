mod front_of_house;
mod back_of_house;

#[cfg(test)]
mod tests {

    pub use crate::front_of_house::hosting;

    pub use crate::back_of_house::cooking;
    #[test]
    fn it_works() {
        let result = hosting::give_two();
        assert_eq!(result, 2);
    }

    #[test]
    fn cooking_works(){
        let result = cooking::give_100();
        assert_eq!(result, 100);
    }
}
