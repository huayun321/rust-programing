fn main() {
    //======= Dereferencing a Raw Pointer =======//
    // let mut num = 5;
    //
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r = address as *const i32;

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    //======= Calling an Unsafe Function or Method =======//
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    //======= Creating a Safe Abstraction over Unsafe Code =======//
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    //won't work
    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    //
    //     assert!(mid <= len);
    //
    //     (&mut values[..mid], &mut values[mid..])
    // }

    use std::slice;

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    //======= Using extern Functions to Call External Code =======//
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    fn main() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    //======= Accessing or Modifying a Mutable Static Variable =======//
    // static HELLO_WORLD: &str = "Hello, world!";
    //
    // fn main() {
    //     println!("name is: {}", HELLO_WORLD);
    // }

    // static mut COUNTER: u32 = 0;
    //
    // fn add_to_count(inc: u32) {
    //     unsafe {
    //         COUNTER += inc;
    //     }
    // }
    //
    // fn main() {
    //     add_to_count(3);
    //
    //     unsafe {
    //         println!("COUNTER: {}", COUNTER);
    //     }
    // }

    //======= Implementing an Unsafe Trait =======//
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // fn main() {}

}
