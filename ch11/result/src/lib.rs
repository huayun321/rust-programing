pub fn add() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() -> Result<(), String> {
        // if 2 + 1 == 4 {
        //     Ok(())
        // } else {
        //     Err(String::from("two plus two does not equal four"))
        // }
        add()?;
        Ok(())
    }
}
