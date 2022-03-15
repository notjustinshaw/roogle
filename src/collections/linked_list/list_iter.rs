use core::marker::PhantomData;
use std::borrow::Borrow;

use super::{ListNode, StrongPointer}; // for cursors

/// An immutable iterator over the elements of a `LinkedList`.
pub struct ListIter<'a, T: Clone> {
    pub current: Option<&'a StrongPointer<ListNode<T>>>,
    pub marker: PhantomData<ListNode<T>>,
}

/// Returns an iterator over the elements of the list.
impl<'a, T: Clone> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    /// Returns the next element of the list.
    fn next(&mut self) -> Option<Self::Item> {
        self.current.borrow().map(|curr| {
            self.current = 
            // old.borrow().data.clone()
        })
    }
}
