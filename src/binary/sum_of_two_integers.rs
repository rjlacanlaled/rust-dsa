pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut x = a.clone();
    let mut y = b.clone();

    if y == 0 {
        return x;
    }

    while y != 0 {
        let carry = x & y;
        x = x ^ y;
        y = carry << 1;
    }

    x
}

#[cfg(test)]
mod sum_of_two_integers_test {
    use super::*;

    #[test]
    pub fn should_return_sum_of_two_integers() {
        assert_eq!(get_sum(1, 2), 3);
    }

    #[test]
    pub fn should_return_sum_of_two_integers_for_edge_case() {
        assert_eq!(get_sum(-2, 3), 1);
    }

    #[test]
    pub fn should_return_sum_of_two_integers_for_edge_case_2() {
        assert_eq!(get_sum(-2, -3), -5);
    }

    #[test]
    pub fn should_return_sum_of_two_integers_for_edge_case_3() {
        assert_eq!(get_sum(0, 0), 0);
    }

    #[test]
    pub fn should_return_sum_of_two_integers_for_edge_case_4() {
        assert_eq!(get_sum(2, 3), 5);
    }
}
