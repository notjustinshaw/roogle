pub mod list_iter;
/// A doubly-linked list from hell >:)
pub mod list_node;

pub use list_iter::ListIter;
pub use list_node::ListNode;
pub use list_node::StrongPointer;

use core::fmt;
use core::marker::PhantomData; // for cursors
use std::cell::RefCell;
use std::clone::Clone;
use std::fmt::Display;
use std::fmt::Formatter;
use std::rc::Rc;

// A doubly-linked list.
//
// This `LinkedList` allows pushing and popping elements at either end.
pub struct LinkedList<T: Clone> {
    pub head: Option<StrongPointer<ListNode<T>>>,
    pub tail: Option<StrongPointer<ListNode<T>>>,
    num_elements: u64,
}

impl<T: Clone> LinkedList<T> {
    // Creates an empty `LinkedList`.
    ///
    /// # Example
    ///
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    /// let list: LinkedList<u32> = LinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            num_elements: 0,
        }
    }

    /// Returns the length of the list.
    ///
    /// # Example
    ///
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// assert_eq!(list.len(), 3);
    /// list.pop_front();
    /// list.pop_front();
    /// list.pop_front();
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.pop_front(), None);
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn len(&self) -> u64 {
        self.num_elements
    }

    /// Adds an element to the head of the list.
    ///
    /// # Example
    ///
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_front(1);
    /// assert_eq!(list.len(), 1);
    /// list.push_front(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn push_front(&mut self, value: T) {
        let new_node: ListNode<T> = ListNode::from(value);
        let new_ref: StrongPointer<ListNode<T>> = Rc::new(RefCell::new(new_node));
        match self.head.take() {
            Some(old_head) => {
                // old head's prev points to the new node (as a weak ptr)
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_ref.clone()));
                new_ref.borrow_mut().next = Some(old_head);
                self.head = Some(new_ref);
            }
            None => {
                // list is empty, so new node is the head and the tail
                self.head = Some(new_ref.clone());
                self.tail = Some(new_ref);
            }
        }
        self.num_elements += 1;
    }

    /// Removes an element from the head of the list and returns it.
    ///
    /// # Example
    ///
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), None);
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.head
            .take()
            .map(|old_head: StrongPointer<ListNode<T>>| {
                self.num_elements -= 1;
                if self.num_elements == 0 {
                    self.head = None;
                    self.tail = None;
                } else {
                    self.head = old_head.borrow_mut().next.take();
                }
                old_head.borrow().data.clone()
            })
    }

    /// Adds an element to the tail of the list.
    ///
    /// # Example
    ///
    /// ```
    /// use roogle::collections::linked_list::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// assert_eq!(list.len(), 0);
    /// list.push_back(1);
    /// assert_eq!(list.len(), 1);
    /// list.push_back(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn push_back(&mut self, value: T) {
        let new_node: ListNode<T> = ListNode::from(value);
        let new_ref: StrongPointer<ListNode<T>> = Rc::new(RefCell::new(new_node));
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_ref.clone());
                new_ref.borrow_mut().prev = Some(Rc::downgrade(&old_tail.clone()));
                self.tail = Some(new_ref);
            }
            None => {
                self.head = Some(new_ref.clone());
                self.tail = Some(new_ref);
            }
        }
        self.num_elements += 1;
    }

    /// Removes an element from the tail of the list and returns it.
    ///
    /// # Example
    ///
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), None);
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail
            .take()
            .map(|old_tail: StrongPointer<ListNode<T>>| {
                self.num_elements -= 1;
                if self.num_elements == 0 {
                    self.head = None;
                    self.tail = None;
                } else {
                    self.tail = old_tail
                        .borrow_mut()
                        .prev
                        .take()
                        .map(|prev| prev.upgrade().unwrap());
                }
                old_tail.borrow().data.clone()
            })
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// # Example
    ///
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<u32> = LinkedList::new();
    ///
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// assert_eq!(list.len(), 3); // list -> 1 -> 2 -> 3
    ///
    /// list.retain(|x| x % 2 == 0); // list -> 2
    ///
    /// assert_eq!(list.len(), 1);
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn retain(&mut self, mut f: impl FnMut(&T) -> bool) {
        let mut current = self.head.take();
        while let Some(curr) = current {
            let node = curr.borrow();
            if !f(&node.data) {
                // We're removing the front: update head pointer
                if node.prev.is_none() {
                    self.head = node.next.clone();
                }

                // We're removing the tail: update tail pointer
                if node.next.is_none() {
                    self.tail = curr
                        .borrow()
                        .prev
                        .clone()
                        .map(|weak_ptr| weak_ptr.upgrade().unwrap());
                }

                // Remove current node: change prev's next and next's prev
                node.prev.as_ref().map(|prev| {
                    prev.upgrade().map(|prev| {
                        prev.borrow_mut().next = node.next.clone().take();
                    });
                });
                node.next.as_ref().map(|next| {
                    next.borrow_mut().prev = node.prev.clone().take();
                });
                self.num_elements -= 1;
            }
            current = node.next.clone();
        }
    }

    /// Clears the linked list, removing all values.
    ///
    /// # Example
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(list.len(), 3);
    /// list.clear();
    /// assert_eq!(list.len(), 0);
    /// assert_eq!(list.pop_front(), None);
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.num_elements = 0;
    }

    /// Returns `true` if the list contains no elements.
    ///
    /// # Example
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// assert!(list.is_empty());
    /// list.push_back(1);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.num_elements == 0
    }

    /// Returns a reference to an element at the given index or `None` if the
    /// index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use roogle::collections::linked_list::LinkedList;
    /// let mut list: LinkedList<u32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// list.push_back(4);
    /// list.push_back(5);
    ///
    /// assert_eq!(list.get(0), Some(1));
    /// assert_eq!(list.get(1), Some(2));
    /// assert_eq!(list.get(2), Some(3));
    /// assert_eq!(list.get(3), Some(4));
    /// assert_eq!(list.get(4), Some(5));
    /// assert_eq!(list.get(5), None);
    /// ```
    pub fn get(&self, index: u64) -> Option<T> {
        if index >= self.num_elements {
            return None;
        }

        // Figure out which end is closer to the index
        if index < self.num_elements / 2 {
            let mut current = self.head.clone();
            for _ in 0..index {
                current = current.unwrap().borrow().next.clone();
            }
            Some(current.unwrap().borrow().data.clone())
        } else {
            let mut current = self.tail.clone();
            for _ in 0..(self.num_elements - index - 1) {
                current = current
                    .unwrap()
                    .borrow()
                    .prev
                    .as_ref()
                    .map(|prev| prev.upgrade().unwrap());
            }
            Some(current.unwrap().borrow().data.clone())
        }
    }

    /// Returns an iterator over the list.
    ///
    /// # Example
    ///
    /// ```
    /// use roogle::collections::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<u32> = LinkedList::new();
    ///
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    ///
    /// assert_eq!(list.len(), 3);
    ///
    /// let mut iter = list.iter();
    ///
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(1));
    ///
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: &self.head,
            marker: PhantomData,
        }
    }
}

impl<T: Clone + Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut iter = self.iter();
        if let Some(data) = iter.next() {
            write!(f, "{}", data)?;
            while let Some(data) = iter.next() {
                write!(f, " -> {}", data)?;
            }
        }
        Ok(())
    }
}