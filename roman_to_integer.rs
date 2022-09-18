use std::collections::HashMap;

//
pub fn roman_to_int(s: String) -> i32 {
    let mut map = HashMap::<&str, i32>::new();
    map.insert("I", 1);
    map.insert("V", 5);
    map.insert("X", 10);
    map.insert("L", 50);
    map.insert("C", 100);
    map.insert("D", 500);
    map.insert("M", 1000);
    map.insert("IV", 4);
    map.insert("IX", 9);
    map.insert("XL", 40);
    map.insert("XC", 90);
    map.insert("CD", 400);
    map.insert("CM", 900);

    let mut sum = 0;

    let mut i = 0;
    while i < s.len() {
        if i + 1 == s.len() {
            sum += map[&s[i..i + 1]];
            break;
        }

        if map.contains_key(&s[i..i + 2]) {
            sum += map[&s[i..i + 2]];
            i += 1;
        } else {
            sum += map[&s[i..i + 1]];
        }
        i += 1;
    }
    sum
}
