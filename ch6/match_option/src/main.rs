fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("the value is: {}", six);
    println!("the value is: {}", none);

}

fn plus_one(x:Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}