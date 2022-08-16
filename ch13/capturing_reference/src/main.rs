fn main() {
    //闭包捕获不可变引用
    // let list1 = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list1);
    //
    // let only_borrows = || println!("From borrows {:?}", list1);
    //
    // println!("Before calling closure: {:?}", list1); //因为闭包是捕获的不可变引用 而不可变引用 可以在多个地方调用
    // only_borrows();
    // println!("After calling closure: {:?}", list1);

    //闭包捕获可变引用
    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);
    //
    // let mut borrows_mutably = || list.push(7);
    //
    // //
    // //println!("Before calling closure: {:?}", list); //^^^^ immutable borrow occurs here
    //
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    //https://mytechshares.com/2021/08/04/fn-fnmut-fnonce/

    //fn
    let s = String::from("董泽润的技术笔记");
    let f = || {println!("{}", s);};
    test(f);

    // //fn mut
    // let mut s = String::from("董泽润的技术笔记");
    // let f = || {s.push_str("不错");};
    // test(f);
    //
    // //fn once
    // let mut s = String::from("董泽润的技术笔记");
    // let f = || {let _ = s;};
    // test(f);
}

fn test<T>(f: T) where
    T: Fn() // T: FnMut // T: FnMut // T: FnOnce
{
    f();
}
