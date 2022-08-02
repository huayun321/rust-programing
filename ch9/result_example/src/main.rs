use std::fs::{self, File};
// use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // match result
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file:{:?}", error)
    //     },
    // };

    // error kind
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => panic!("There was a problem opening the file: {:?}", other_error),
    //     },
    // };

    //map error
    // let f = File::open("hello.txt").map_err(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error);
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // });

    //unwrap expect
    // let f = File::open("hello1.txt").unwrap();

    // let f = File::open("hello1.txt").expect("Failed to open hello.txt");
    let s = read_username_from_file().unwrap();
    println!("{:?}", s);
}
//
// fn read_username_from_file() ->Result<String, io::Error> {
//     let f = File::open("hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

//?
// fn read_username_from_file() ->Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

//method chain
// fn read_username_from_file() ->Result<String, io::Error> {
//     let mut s = String::new();
//
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//
//     Ok(s)
// }

//
fn read_username_from_file() ->Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}