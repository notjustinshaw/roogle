use roogle::collections::linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    println!("list len: {}", list.len());

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);

    println!("list: {}", list);
    println!("new len: {}", list.len());
}
