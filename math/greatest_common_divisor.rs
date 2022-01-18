pub fn greatest_common_divisor_recursive(a: i64, b: i64) -> i64 {
    if a == 0{
        b.abs()
    }
    else{
        greatest_common_divisor_recursive(b % a, a)
    }
}

pub fn greatest_common_divisor_iter(mut a: i64, mut b: i64) -> i64{
    while a != 0{
        let remainder = b % a;
        b = a;
        a = remainder;
    }
    b.abs()
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn positive_number_recursive(){
        assert_eq!(greatest_common_divisor_recursive(4, 16), 4);
    }

    #[test]
    fn positive_number_iter(){
        assert_eq!(greatest_common_divisor_iter(4, 16), 4);
        assert_eq!(greatest_common_divisor_iter(3, 5), 1);
    }
}

