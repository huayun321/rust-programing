fn main() {
    let s1 = String::from("hello");

    let len = calculate_length2(&s1);
    println!("The value of len is: {}", len);

    let (s2, len) = calculate_length(s1);
    println!("The value of s1 is: {}", s2);
    println!("The value of len is: {}", len);
}

fn calculate_length2(s: &String) -> usize { // s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
