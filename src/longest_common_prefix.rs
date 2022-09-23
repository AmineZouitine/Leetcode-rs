pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return "".to_string();
    }

    let min = strs
        .iter()
        .min_by(|x: &&String, y: &&String| x.len().cmp(&y.len()))
        .unwrap();

    let mut string_longest = String::new();

    for i in 0..min.len() {
        let current_letter = min.as_bytes()[i];

        let elements: Vec<&String> = strs
            .iter()
            .filter(|x| x.as_bytes()[i] == current_letter)
            .collect();

        if elements.len() == strs.len() {
            string_longest = format!("{}{}", string_longest, current_letter as char);
        } else {
            break;
        }
    }

    string_longest
}
