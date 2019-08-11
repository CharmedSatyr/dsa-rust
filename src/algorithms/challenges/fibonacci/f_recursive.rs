#[allow(dead_code)]
pub fn f_recursive(num: u32) -> u32 {
    if num == 0 {
       return 0
    } else if num == 1 {
       return 1
    }
    f_recursive(num - 1) + f_recursive(num - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_of_zero() {
        assert_eq!(f_recursive(0), 0);
    }

    #[test]
    fn f_of_one() {
        assert_eq!(f_recursive(1), 1);
    }

    #[test]
    fn f_of_eight() {
        assert_eq!(f_recursive(8), 21);
    }

    #[test]
    fn f_of_twenty() {
        assert_eq!(f_recursive(20), 6765);
    }
}
