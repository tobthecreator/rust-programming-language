mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

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
}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

pub fn eat_at_restaurant() {
    // In the original version, we cannot call this function
    // because hosting is not public.front_of_house

    // Modules define the 'privacy boundary'.
    // All functions, enums, modules and constants are private by default.
    // Parents cannot use private items of children, but children can view and use private
    // implementations of parents.

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative
    front_of_house::hosting::add_to_waitlist();
}
