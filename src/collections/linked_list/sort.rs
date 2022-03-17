use std::fmt::Display;

use super::LinkedList;

/// A really slow sort algorithm.
pub fn bubble_sort<T: Clone + Display>(
    list: &mut LinkedList<T>,
    mut compare: impl FnMut(&T, &T) -> bool,
) {
    print(list, 10);
    let len = list.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            let first = list.get_ptr(j).unwrap();
            let second = list.get_ptr(j + 1).unwrap();
            if compare(&first.borrow().data, &second.borrow().data) {
                if second.borrow().next.is_none() {
                    list.tail = Some(first.clone());
                }
                first.borrow_mut().next = second.borrow().next.clone();
                second.borrow_mut().next = Some(second.clone());
                first.swap(&second);
            }
        }
    }
    print(list, 10);
}

// print with a cycle cap
fn print<T: Clone + Display>(list: &mut LinkedList<T>, max_depth: u32) {
    let mut iter = list.iter();
    if let Some(first) = iter.next() {
        print!("{}", first);
        for _ in 0..max_depth {
            if let Some(next) = iter.next() {
                print!(" -> {}", next);
            }
        }
    }
    println!();
}
