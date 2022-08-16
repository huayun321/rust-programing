use std::thread;
use std::time::Duration;
// 类型推断与注释
fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(1);

    // fn  add_one_v1 (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // 根据参数推断
    let add_one_v3 = |x| { x + 1 };
    // 编译的时候需要这一行
    add_one_v3(1);
    // 根据参数推断
    let add_one_v4 = |x| x + 1  ;
    // 编译的时候需要这一行
    add_one_v4(1);

    // 这样是不能通过编译的
    // let example_closure = |x| x;
    //
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5); //expected struct `String`, found integer

    println!("Hello, world!");
}
