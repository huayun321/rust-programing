fn main() {
    //通过使用let声明同名变量 并保持不可变性
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    //由于使用的是同名的新变量 我们可以改变变量的类型
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
