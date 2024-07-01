use std::fmt::Result;
use std::io::Result as IoResult;

mod front_of_house;

fn serve_orders(){}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_orders();
    }
    
    fn cook_order() {}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {

    // Absolute path
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Sourdough");
    meal.toast = String::from("Gluten-Free");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Relative path
    hosting::add_to_waitlist();
}