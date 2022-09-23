// O(n)
pub fn is_palindrome(x: i32) -> bool {
    let size = x.to_string().len();
    let mut end = size - 1;
    let x = x.to_string();

    for i in 0..size {
        if i >= end {
            break;
        }
        if x.as_bytes()[i] != x.as_bytes()[end] {
            return false;
        }
        end -= 1;
    }
    return true;
}
