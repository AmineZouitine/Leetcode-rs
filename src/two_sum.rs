use std::collections::HashMap;

// Solution 1
// O(n)
pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, usize>::new();

    for x in 0..nums.len() {
        map.insert(nums[x], x);
    }

    for x in 0..nums.len() {
        let map_target = target - nums[x];
        if map.contains_key(&map_target) && map[&map_target] != x {
            return vec![x as i32, map[&map_target] as i32];
        }
    }

    panic!("Value not found !");
}

// Solution 2
// O(n²)
pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for x in 0..nums.len() {
        for y in x + 1..nums.len() {
            println!("{} ", y);
            if nums[x] + nums[y] == target {
                return vec![x as i32, y as i32];
            }
        }
    }
    panic!("Value not found !");
}
