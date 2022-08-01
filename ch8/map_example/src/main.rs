use std::collections::HashMap;

fn main() {
    //create
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    //ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    // println!("field_name {}", field_name); //value borrowed here after move

    //index
    let score = scores.get(&String::from("Blue"));
    // let score = scores.get("Blue"); //^^^ the trait `Borrow<str>` is not implemented for `&String`
    println!("{:?}", score);

    for (k, v) in scores {
        println!("{} {}", k, v)
    }

    //update
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // update or
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // base on  old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
