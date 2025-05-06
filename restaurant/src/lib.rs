mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }

    mod serving {
        fn take_orders() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what brea we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // Absolute path:
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // meal.seasonal_fruit = String::from("blueberries");
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // If we make an enum public, all the variants will be public
    // Por lo general, los enums tienen bastantes variantes o literales, sería la mar de castroso
    // tener que anotar todas las variantes de manera individual.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // We go to the parent module...
        super::deliver_order();
    }

    fn cook_order() {}
}
