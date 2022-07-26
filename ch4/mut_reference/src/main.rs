fn main() {
    //使用可变引用可以修改 原值
    let mut s = String::from("hello");
    change(&mut s);
    println!("the value of s is: {}", s);


    //同一时间只能有一个可变引用
    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // println!("{}, {}", r1, r2);

    //多个作用域可以有多个可变引用
    // let mut s = String::from("hello");
    //
    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.
    //
    // let r2 = &mut s;

    //不能在有不可变引用的时候 有可变引用 避免 data race
    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    // println!("{}, {}, and {}", r1, r2, r3);

}

fn change(s:&mut String) {
    s.push_str(", world!");
}
