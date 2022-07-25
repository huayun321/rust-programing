fn main() {
    let five = five();
    println!("The value of five is: {}", five);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn five() ->i32 {
    5
}

fn plus_one(x:i32) ->i32 {
    x + 1
}
