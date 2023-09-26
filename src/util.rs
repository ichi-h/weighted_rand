pub mod math {
    fn gcd(a: u32, b: u32) -> u32 {
        let (a, b) = if a < b { (b, a) } else { (a, b) };

        if a % b == 0 {
            b
        } else {
            gcd(b, a % b)
        }
    }

    pub fn gcd_for_slice(slice: &[u32]) -> u32 {
        if slice.is_empty() {
            return 0;
        }

        let mut iter = slice.iter().skip_while(|x| x == &&0);
        let first = match iter.next() {
            Some(v) => *v,
            None => return 1
        };

        let gcd = iter.fold(
            first,
            |acc, cur| {
                if *cur == 0 {
                    acc
                } else {
                    gcd(*cur, acc)
                }
            },
        );
        gcd
    }
}

#[cfg(test)]
mod math_test {
    use crate::util::math::gcd_for_slice;

    #[test]
    fn gcd_pattern_1() {
        let actual = gcd_for_slice(&[4, 20, 32]);
        assert_eq!(actual, 4)
    }

    #[test]
    fn gcd_pattern_2() {
        let actual = gcd_for_slice(&[77, 9, 25]);
        assert_eq!(actual, 1)
    }

    #[test]
    fn gcd_pattern_3() {
        let actual = gcd_for_slice(&[11, 0, 22]);
        assert_eq!(actual, 11)
    }

    #[test]
    fn gcd_pattern_4() {
        let actual = gcd_for_slice(&[]);
        assert_eq!(actual, 0)
    }
}
