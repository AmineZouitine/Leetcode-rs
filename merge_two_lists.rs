#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if let None = list1 {
        return list2;
    } else if let None = list2 {
        return list1;
    }

    let val1 = list1.as_ref().unwrap().val;
    let val2 = list2.as_ref().unwrap().val;

    if val1 < val2 {
        return Some(Box::new(ListNode {
            val: val1,
            next: merge_two_lists(list1.unwrap().next, list2),
        }));
    }
    return Some(Box::new(ListNode {
        val: val2,
        next: merge_two_lists(list1, list2.unwrap().next),
    }));
}
