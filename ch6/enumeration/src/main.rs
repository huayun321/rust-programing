fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    home.route();
    loopback.route();

    let n = Message::None;
    let m = Message::Move {x: 111, y: 222};
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(11,22,33);

    n.call();
    m.call();
    w.call();
    c.call();
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddr {
    fn route(&self) {
        println!("the value is: {:?}", self);
    }
}

#[derive(Debug)]
enum Message {
    None,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("the value is: {:?}", self)
    }
}