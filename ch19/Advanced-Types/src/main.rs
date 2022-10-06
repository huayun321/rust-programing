fn main() {
    //========  Using the Newtype Pattern for Type Safety and Abstraction ========//
    struct Millimeters(u32);
    struct Meters(u32);

    //======== Creating Type Synonyms with Type Aliases ========//
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);


    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }
    //======== The Never Type that Never Returns ========//
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };

    impl<T> Option<T> {
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }

    print!("forever ");

    loop {
        print!("and ever ");
    }

    //======== Dynamically Sized Types and the Sized Trait ========//
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    //trait

    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
}
