static mut DP: [i32; 45] = [0; 45];

pub fn climb_stairs(n: i32) -> i32 {
    unsafe {
        match n {
            1 => {
                return 1;
            }
            2 => {
                return 2;
            }
            _ => {
                let i = (n - 1) as usize;
                let memo = DP[i];

                match memo {
                    0 => {
                        let result = climb_stairs(n - 1) + climb_stairs(n - 2);
                        DP[i] = result;
                        return result;
                    }
                    _ => {
                        return memo;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod climbing_stairs_test {
    use super::*;

    #[test]
    pub fn should_return_climb_stairs() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    pub fn should_return_climb_stairs_for_edge_case() {
        assert_eq!(climb_stairs(1), 1);
    }

    #[test]
    pub fn should_return_climb_stairs_for_edge_case_2() {
        assert_eq!(climb_stairs(2), 2);
    }
}
