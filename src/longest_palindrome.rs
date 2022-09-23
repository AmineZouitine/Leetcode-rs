pub fn longest_palindrome(s: String) -> String {
    let mut max_sub: &str = "";

    for i in 0..s.len() {
        let mut l = i as i32;
        let mut r = i as i32;
        while l >= 0
            && (r as usize) < s.len()
            && s.as_bytes()[l as usize] == s.as_bytes()[r as usize]
        {
            if (r + 1 - l) as usize > max_sub.len() {
                max_sub = &s[l as usize..=r as usize];
            }
            l -= 1;
            r += 1;
        }

        let mut l = i as i32;
        let mut r = (i + 1) as i32;
        while l >= 0
            && (r as usize) < s.len()
            && s.as_bytes()[l as usize] == s.as_bytes()[r as usize]
        {
            if (r + 1 - l) as usize > max_sub.len() {
                max_sub = &s[l as usize..=r as usize];
            }
            l -= 1;
            r += 1;
        }
    }

    max_sub.to_string()
}
