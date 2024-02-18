mod pizza_order {
    pub struct Pizza {
        pub topping: String,
        pub cheese: String,
        pub dough: String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                topping: String::from(topping),
                cheese: String::from("mozzarella"),
                dough: String::from("regular dough"),
            }
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }
        pub fn take_order() {
            seat_at_table();

            let pizza = super::Pizza::lunch("pepperoni");
            println!("Customer is having {}", pizza.topping);
            println!("Customer is having {}", pizza.cheese);
            println!("Customer is having {}", pizza.dough);
            serve_customer(pizza);
        }
        fn serve_customer(pizza: super::Pizza) {
            println!("Customer served regular pizza with {}", pizza.topping);
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
