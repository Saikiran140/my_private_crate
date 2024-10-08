pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greet_from_sivi(name: &str) -> String {
     let mut greeting = String::from("Hello ");
     greeting.push_str(name);
     greeting.push_str(" Welcome to Sivi !!!");
     greeting
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
