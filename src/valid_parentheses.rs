fn get_opposite(element: char) -> char {
    match element {
        ']' => '[',
        ')' => '(',
        '}' => '{',
        _ => panic!("Unexpeted char"),
    }
}

fn is_open_element(element: char) -> bool {
    return element == '(' || element == '[' || element == '{';
}

fn is_close_element(element: char) -> bool {
    return element == ')' || element == ']' || element == '}';
}

pub fn is_valid(s: String) -> bool {
    let mut open_elements = Vec::<char>::new();
    for x in s.chars() {
        if !is_open_element(x) && !is_close_element(x) {
            continue;
        }
        if is_open_element(x) {
            open_elements.push(x);
        } else if open_elements.is_empty() || get_opposite(x) != open_elements.pop().unwrap() {
            return false;
        }
    }
    open_elements.is_empty()
}
