//Solution 1
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut incremented_vec = Vec::<i32>::new();
    let last_digit: i32;
    match digits.is_empty() {
        true => last_digit = 0,
        false => last_digit = digits[digits.len() - 1],
    }
    match last_digit {
        9 => {
            incremented_vec.append(&mut plus_one(digits[0..digits.len() - 1].to_vec()));
            incremented_vec.push(0);
        }
        v => {
            if !digits.is_empty() {
                incremented_vec.extend_from_slice(&digits[0..digits.len() - 1]);
            }
            incremented_vec.push(v + 1);
        }
    }
    incremented_vec
}
