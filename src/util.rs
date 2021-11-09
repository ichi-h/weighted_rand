pub mod math {
    fn gcd(a: u32, b: u32) -> u32 {
        let (a, b) = if a < b { (b, a) } else { (a, b) };

        if a % b == 0 {
            b
        } else {
            gcd(b, a % b)
        }
    }

    pub fn gcd_for_vec(vector: &Vec<u32>) -> u32 {
        if vector.is_empty() {
            return 0;
        }

        let first = vector[0];
        vector.iter().fold(
            first,
            |acc, cur| {
                if *cur == 0 {
                    acc
                } else {
                    gcd(*cur, acc)
                }
            },
        )
    }
}

#[cfg(test)]
mod math_test {
    use crate::util::math::gcd_for_vec;

    #[test]
    fn gcd_pattern_1() {
        let actual = gcd_for_vec(&vec![4, 20, 32]);
        assert_eq!(actual, 4)
    }

    #[test]
    fn gcd_pattern_2() {
        let actual = gcd_for_vec(&vec![77, 9, 25]);
        assert_eq!(actual, 1)
    }

    #[test]
    fn gcd_pattern_3() {
        let actual = gcd_for_vec(&vec![11, 0, 22]);
        assert_eq!(actual, 11)
    }

    #[test]
    fn gcd_pattern_4() {
        let actual = gcd_for_vec(&Vec::new());
        assert_eq!(actual, 0)
    }
}
