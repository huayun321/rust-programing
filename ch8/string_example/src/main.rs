fn main() {
    //create
    let data = "initial contents";
    let s = data.to_string();
    // let s = String::from("initial contents");
    println!("the value of s is: {}", s);


    //update
    let mut s = String::new();
    s.push_str("hello");
    let s2 = ", world!";
    s.push_str(s2);
    println!("the value of s is: {}", s);
    println!("the value of s2 is: {}", s2);

    let mut s= String::from("lo");
    s.push('l');
    println!("the value of s is: {}", s);


    //concatenate
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("the value of s3 is: {}", s3); //s1  drop

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("the value of s is: {}", s);
    println!("the value of s1 is: {}", s1);
    println!("the value of s2 is: {}", s2);
    println!("the value of s3 is: {}", s3);

    //index
    let s1 = String::from("Hello");
    // let h = s1[0]; //`String` cannot be indexed by `{integer}`
    let len = s1.len();
    println!("the value of len is: {}", len);

    let s1 = String::from("你好啊");
    let len = s1.len();
    println!("the value of len is: {}", len);

    // let n = &s1[0];//`String` cannot be indexed by `{integer}`
    let ss = &s1[0..3];
    println!("the value of ss is: {}", ss);

    for c in s1.chars() {
        println!("{}", c);
    }
    for c in s1.bytes() {
        println!("{}", c);
    }
}
