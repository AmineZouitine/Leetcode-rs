pub fn max_area(height: Vec<i32>) -> i32 {
    fn calcul_area(height: &Vec<i32>, min_index: usize, max_index: usize) -> i32 {
        let mut min = height[max_index];
        if height[min_index] < height[max_index] {
            min = height[min_index];
        }
        return min * (max_index - min_index) as i32;
    }

    let mut max = 0;

    let mut left = 0;
    let mut right = height.len() - 1;

    while left != right {
        let current_area = calcul_area(&height, left, right);
        if current_area > max {
            max = current_area;
        }

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max
}
