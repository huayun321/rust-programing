fn main() {
    let p = Coin::Penny;
    let n = Coin::Nickle;
    let d = Coin::Dime;
    let q1 = Coin::Quarter(UsState::Alabama);
    let q2 = Coin::Quarter(UsState::Alaska);

    println!("the value in cents: {}", value_in_cents(p));
    println!("the value in cents: {}", value_in_cents(n));
    println!("the value in cents: {}", value_in_cents(d));

    println!("the value in cents: {}", value_in_cents(q1));
    println!("the value in cents: {}", value_in_cents(q2));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}!", state);
            25
        },
    }
}