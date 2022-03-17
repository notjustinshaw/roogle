use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::{LinkedList, ListNode};

/// A really slow sort algorithm.
pub fn bubble_sort<T: Clone + Display>(
    list: &mut LinkedList<T>,
    mut compare: impl FnMut(&T, &T) -> bool,
) {
    print_reverse(list.tail.as_ref(), 10);
    println!();
    let len = list.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            let first = list.get_ptr(j).unwrap();
            let second = list.get_ptr(j + 1).unwrap();
            if compare(&first.borrow().data, &second.borrow().data) {
                second.borrow_mut().prev = first.borrow().prev.clone();
                first.borrow_mut().prev = Some(Rc::downgrade(&first.clone()));
                first.borrow_mut().next = second.borrow().next.clone();
                second.borrow_mut().next = Some(second.clone());
                first.swap(&second);
            }
            print_reverse(list.tail.as_ref(), 10);
            println!();
        }
    }
}

// print with a cycle cap
fn _print<T: Clone + Display>(list: &mut LinkedList<T>, max_depth: u32) {
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

// print with a cycle cap
fn print_reverse<T: Clone + Display>(node: Option<&Rc<RefCell<ListNode<T>>>>, max_depth: u32) {
    if let Some(list_node) = node {
        if let None = list_node.borrow().prev {
            print!("{}", list_node.borrow().data);
        } else if max_depth > 0 {
            print_reverse(
                list_node.borrow().prev.as_ref().unwrap().upgrade().as_ref(),
                max_depth - 1,
            );
            print!(" <- {}", list_node.borrow().data);
        }
    }
}
