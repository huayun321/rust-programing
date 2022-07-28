mod front_of_house;

//absolute relative path
// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     front_of_house::hosting::add_to_waitlist();
// }


fn serv_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serv_order();
    }

    fn cook_order() {}
}

//use
//absolute
// use crate::front_of_house::hosting;
//relative
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // // 选择黑面包作为夏季早餐
    // let mut meal = back_of_house::Breakfast::summer("Rye");
    // // 修改我们想要的面包类型
    // meal.toast = String::from("Wheat");
    // println!("I'd like {} toast please", meal.toast);
    //
    // //私有不可修改 报错
    // //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    //use
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}