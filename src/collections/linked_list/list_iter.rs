use core::marker::PhantomData;

use super::{ListNode, StrongPointer};  // for cursors

/// An immutable iterator over the elements of a `LinkedList`.
pub struct ListIter<T: Clone> {
  pub current: Option<StrongPointer<ListNode<T>>>,
  pub marker: PhantomData<ListNode<T>>,
}

/// Returns an iterator over the elements of the list.
impl<T: Clone> Iterator for ListIter<T> {
  type Item = T;

  /// Returns the next element of the list.
  fn next(&mut self) -> Option<Self::Item> {
    self.current.take().map(|old: StrongPointer<ListNode<T>>| {
      self.current = old.borrow_mut().next.take();
      old.borrow().data.clone()
    })
  }
}