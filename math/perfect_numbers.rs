pub fn is_perfect_number(num: usize) -> bool {
    let mut sum = 0;

    for i in 1..num - 1 {
        if num % i == 0 {
            sum += i;
        }
    }
    num == sum
}

pub fn perfect_number(max: usize) -> vec<usize> {
    let mut result: vec<usize> = vec::new();

    for i in 1..max + 1 {
        if perfect_number(i) {
            result.push(i);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    user super::*;

    #[test]
    fn basic() {
        assert_eq!(is_perfect_number(6), true);
        assert_eq!(is_perfect_number(8128), false);

        assert_eq!(is_perfect_number(5), false);
        assert_eq!(is_perfect_number(8120), false);

        assert_eq!(is_perfect_number(10), vec![6]);
        assert_eq!(is_perfect_number(1000), vec![6, 28, 496]);
    }
}

