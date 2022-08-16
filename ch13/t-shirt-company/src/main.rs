//闭包捕获环境的例子
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    //注意 self.most_stocked() 中的self是从环境中捕获的。。。
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_num = 0;
        let mut blue_num = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_num += 1,
                ShirtColor::Blue => blue_num +=1,
            }
        }
        if red_num > blue_num {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_perf1 = ShirtColor::Red;
    let giveaway1 = store.giveaway(Some(user_perf1));
    println!(
        "The user with preference {:?} gets {:?}",
        user_perf1, giveaway1
    );
}