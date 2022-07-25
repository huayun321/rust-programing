fn main() {
    let s1 = gives_ownership();
    println!("The value of s1 is: {}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); //s2被废弃
    println!("The value of s3 is: {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello"); //some_string 进入作用域
    some_string // 作为返回值移动至调用函数
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string // 作为返回值移动至调用函数
}