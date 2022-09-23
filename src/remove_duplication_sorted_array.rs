use core::num;

// first solution O(n**2)
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut new_vec = Vec::<i32>::new();

    nums.retain(|element| {
        if new_vec.contains(element) {
            return false;
        }
        new_vec.push(*element);
        return true;
    });

    return nums.len() as i32;
}

// second solution
// O(n)
pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut diff_count = 0;

    for i in 0..nums.len() {
        if i + 1 < nums.len() && nums[i] == nums[i + 1] {
            continue;
        }

        nums[diff_count] = nums[i];
        diff_count += 1;
    }

    diff_count as i32
}
