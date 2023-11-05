pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;

    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let area = ((right - left) as i32) * std::cmp::min(height[left], height[right]);

        if area > max_area {
            max_area = area;
        }

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod container_with_most_water_test {
    use super::*;

    #[test]
    pub fn should_return_max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    pub fn should_return_max_area_for_edge_case() {
        let height = vec![1, 1];
        assert_eq!(max_area(height), 1);
    }

    #[test]
    pub fn should_return_max_area_for_edge_case_2() {
        let height = vec![4, 3, 2, 1, 4];
        assert_eq!(max_area(height), 16);
    }

    #[test]
    pub fn should_return_max_area_for_edge_case_3() {
        let height = vec![1, 2, 1];
        assert_eq!(max_area(height), 2);
    }

    #[test]
    pub fn should_return_max_area_for_edge_case_4() {
        let height = vec![2, 3, 4, 5, 18, 17, 6];
        assert_eq!(max_area(height), 17);
    }
}
