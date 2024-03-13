mod front_of_house;

mod back_of_house;

// Defaulted to private scope
// use crate::front_of_house::hosting

// Public to other services that use this library
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // In scope via use keyowrd
    hosting::add_to_waitlist();

    // Order a breakfast in summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about the kind of bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}
