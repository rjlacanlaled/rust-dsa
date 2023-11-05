pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut shifted_value = b;
    let mut result = 0;

    while shifted_value != 0 {
        result = a ^ shifted_value;
        shifted_value = (a & shifted_value) << 1;
    }

    result
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
}
