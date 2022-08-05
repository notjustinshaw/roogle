use roogle::collections::doubly_linked_list::LinkedList;

#[test]
fn create_empty_list() {
    let list: LinkedList<u32> = LinkedList::new();
    assert_eq!(list.len(), 0);
}

#[test]
fn push_and_pop_front() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
}

#[test]
fn push_and_pop_back() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_back(), None);
}

#[test]
fn push_and_pop_front_and_back() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(1);
    list.push_back(2);
    list.push_front(3);
    list.push_back(4);
    assert_eq!(list.len(), 4);
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_back(), Some(4));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);
}

#[test]
fn iterable() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn clear() {
    let mut list: LinkedList<u32> = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    list.clear();
    assert_eq!(list.len(), 0);
}

#[test]
fn retains_odd_values() {
    let mut list: LinkedList<u64> = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    list.retain(|x| x % 2 != 0);
    for _ in 0..list.len() {
        assert!((list.pop_front().unwrap() % 2) != 0);
    }
    assert_eq!(list.len(), 0);
}

#[test]
fn immutable_iterator() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert_eq!(list.len(), 0);

    list.push_back(1);
    assert_eq!(format!("{}", list), "1");
    assert_eq!(list.len(), 1);

    list.push_back(2);
    assert_eq!(format!("{}", list), "1 -> 2");
    assert_eq!(list.len(), 2);

    list.push_back(3);
    assert_eq!(format!("{}", list), "1 -> 2 -> 3");
    assert_eq!(list.len(), 3);

    list.push_back(4);
    assert_eq!(format!("{}", list), "1 -> 2 -> 3 -> 4");
    assert_eq!(list.len(), 4);

    list.push_back(5);
    assert_eq!(format!("{}", list), "1 -> 2 -> 3 -> 4 -> 5");
    assert_eq!(list.len(), 5);
}
