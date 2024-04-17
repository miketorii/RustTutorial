pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn prints_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_will_pass(){
        let value = prints_and_return_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn it_will_fail(){
        let value = prints_and_return_10(8);
        assert_eq!(5, value);
    }
}
