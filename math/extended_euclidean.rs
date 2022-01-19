fn update_step(a: $mut i32,  old_a: $mut i32, quotient: i32){
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

pub fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32){
    let (mut old_r, mut rem) = (a ,b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0{
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
    }
    (old_r, old_s, old_t)
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn basic(){
        assert_eq!(extended_euclidean(101, 13),(1, 4, -31));
    }
}
