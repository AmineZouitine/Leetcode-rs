use std::collections::HashSet;
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut visited = HashSet::<u8>::new();
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    while i < s.len() && j < s.len() {
        if !visited.contains(&s.as_bytes()[j]) {
            visited.insert(s.as_bytes()[j]);
            count += 1;
            j += 1;
        } else {
            visited.remove(&s.as_bytes()[i]);
            i += 1;
            count -= 1;
        }
        if max < count {
            max = count;
        }
    }
    max
}
