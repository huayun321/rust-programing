fn main() {
    let r1 = Rectangle{
        width: 50,
        height: 100,
    };
    println!("r1 info: {:#?}", r1);
    println!("r1's area is: {}", r1.area());


    let r2 = Rectangle{
        width: 40,
        height: 90,
    };
    println!("r2 info: {:#?}", r2);
    println!("r2's area is: {}", r2.area());

    println!("r1 can hold r2: {}", r1.can_hold(&r2));

    let s1 = Rectangle::square(10);
    println!("s1 info: {:#?}", s1);
    println!("s1's area is: {}", s1.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size:u32) -> Rectangle {
        Rectangle{width: size, height:size}
    }
}