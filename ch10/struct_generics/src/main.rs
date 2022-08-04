fn main() {
    let both_integer = Point{x:5, y:10};
    let both_float = Point{x:1.0, y:4.0};
    let integer_and_float = Point{x:5, y:4.0};
    println!("the value is: {:?}", both_integer);
    println!("the value is: {:?}", both_float);
    println!("the value is: {:?}", integer_and_float);

    let p1 = Point {x:5, y:10.4};
    let p2 = Point {x: "Hello", y:'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
//
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }


#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T,U> Point<T, U> {
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}