pub fn prints_and_return(a: i32) -> i32 {
    println!("the value is: {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn this_test_will_pass() {
        let result = prints_and_return(4);
        assert_eq!(result, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let result = prints_and_return(8);
        assert_eq!(result, 4);
    }
}

//cargo test -- --show-output