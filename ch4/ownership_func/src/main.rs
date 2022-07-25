fn main() {
    let s = String::from("hello"); // 变量 s 进入作用域
    takes_ownership(s); // s 的值被移动进了函数 所以从这里开始它不再有效
    // println!("The value of s is: {}", s1); s has moved

    let x = 3; //变量x 进入作用域
    makes_copy(x); //变量x 同样被传递进了函数 但由于 i32是copy的 之后还是可以正常使用
    println!("The value of x is: {}", x);
} // x首先离开作用域 随后是 s
//但是由于s已的值已经发生了移动，所以没啥特别的事发生

fn takes_ownership(some_string:String) { //some_string 进入作用域
    println!("some string {}", some_string);
} // some_string离开作用域 drop函数被自动调用
// some_string 所占用的内存也随之被释放了

fn makes_copy(some_integer:i32) { //some_integer 进入作用域
    println!("some integer {}", some_integer);
} // some_integer 离开了作用域 没什么特别的事发生
