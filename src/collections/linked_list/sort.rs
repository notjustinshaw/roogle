// use std::fmt::Display;

// use super::LinkedList;

// /// A really slow sort algorithm.
// pub fn bubble_sort<T: Clone + Display>(
//     list: &mut LinkedList<T>,
//     mut compare: impl FnMut(&T, &T) -> bool,
// ) {
//     for i in 0..*list.len() {
//         for j in 0..*list.len() - 1 - i {
//             let first = list.get_ptr(j);
//             let second = list.get_ptr(j + 1);
//             if compare(
//                 &first.as_ref().unwrap().borrow().data,
//                 &second.as_ref().unwrap().borrow().data,
//             ) {
//                 let first_node = first.as_ref().unwrap().borrow();
//                 let second_node = second.as_ref().unwrap().borrow();
//                 let mut first_node_mut = first.as_ref().unwrap().borrow_mut();
//                 let mut second_node_mut = second.as_ref().unwrap().borrow_mut();

//                 // We're changing the front: update head pointer
//                 if first_node.prev.is_none() {
//                     list.head = second.clone();
//                 }

//                 // We're changing the tail: update tail pointer
//                 if second_node.next.is_none() {
//                     list.tail = first.clone();
//                 }

//                 // Swap the pointers: Remove first_node, then insert first_node
//                 let prev_ptr = first_node.prev.as_ref().unwrap().clone();
//                 first_node.prev.as_ref().map(|maybe_prev| {
//                     maybe_prev.upgrade().map(|prev| {
//                         prev.borrow_mut().next = first_node.next.clone();
//                     });
//                 });
//                 first_node.next.as_ref().map(|next| {
//                     next.borrow_mut().prev = first_node.prev.clone();
//                 });
//                 first_node_mut.next = second_node.next.clone();
//                 first_node_mut.prev = second_node
//                     .next
//                     .as_ref()
//                     .map(|next| next.borrow().prev.as_ref().unwrap().clone());
//                 second_node
//                     .next
//                     .as_ref()
//                     .map(|next| next.borrow_mut().prev = Some(prev_ptr));
//                 second_node_mut.next = first.clone();
//             }
//         }
//     }
// }
