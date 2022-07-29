fn main() {
    //create
    let v:Vec<i32> = Vec::new();
    let v1 = vec![1,2,3];
    //update
    let mut v = Vec::new();
    for i in 1..5 {
        v.push(i);
    }
    println!("the value of v is: {:?}", v);
    //Dropping a Vector Drops Its Elements
    {
        let x = vec![1,2,3];
    } // <- x goes out of scope and is freed here

    //read
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("the third element is: {}", third);

    match v.get(2) {
        Some(i) => println!("the third element is: {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];// will panic
    let does_not_exist = v.get(100);
    println!("does_not_exist is: {:?}", does_not_exist);

    //borrow
    let mut v = vec![1,2,3,4,5];

    let first = &v[0];

    // v.push(6); //mutable borrow occurs here
    println!("the first element is: {}", first);

    //iteration
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    //enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:?}", i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


