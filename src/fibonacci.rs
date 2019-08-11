pub fn f_loop(num: u32) -> u32 {
    let mut n = num.clone();

    let mut a = 0;
    let mut b = 1;

    while n >= 1 {
        let tmp = a + b;
        a = b;
        b = tmp;
        n -= 1;
    }
    a
}



#[cfg(test)]
mod tests {
    use crate::fibonacci::f_loop;
    #[test]
    fn f_of_zero() {
        assert_eq!(f_loop(0), 0);
    }

    #[test]
    fn f_of_one() {
        assert_eq!(f_loop(1), 1);
    }

    #[test]
    fn f_of_eight() {
        assert_eq!(f_loop(8), 21);
    }

    #[test]
    fn f_of_twenty() {
        assert_eq!(f_loop(20), 6765);
    }
}
