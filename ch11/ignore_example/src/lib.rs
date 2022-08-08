#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

//cargo test
//cargo -- --ignored