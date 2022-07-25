fn main() {
    // MOVE
    let x = 5;
    let y = x;
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    //println!("The value of s1 is: {}", s1); // s1 已将所有权移交给s2 之后被废弃 此处引用会报错 ^^ value borrowed here after move
    println!("The value of s2 is: {}", s2);

    //CLONE
    let s1 = String::from("hello"); //shadow
    let s2 = s1.clone();
    println!("The value of s1 is: {}", s1);
    println!("The value of s2 is: {}", s2);
}
