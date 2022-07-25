fn main() {
    let _y = 6; //statement
    // let x = (let y = 6); //语句没有返回值

    let x = 5;

    let y = {
        let x= 3; //shadow
        x + 1
    };

    println!("The value of y is: {}", y);
    //语句没有返回值
    //表达式有返回值
    //函数 宏 {}作用域 5+6 6 这些都是表达式
}
