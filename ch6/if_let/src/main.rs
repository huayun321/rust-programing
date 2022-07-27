fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("other");
    }

    let p = Coin::Penny;
    let q = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = q {
        println!("State quarter form {:?}!", state);
    } else {
        println!("not quarter!");
    }
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