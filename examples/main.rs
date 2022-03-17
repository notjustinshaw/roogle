use roogle::collections::linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(2);
    list.push_back(1);
    list.push_back(4);
    list.push_back(5);
    list.push_back(3);

    println!("list len: {}", list.len());
    list.sort(|a, b| a > b);

    println!("{:?}", list.pop_back());
    println!("{:?}", list.pop_back());
    println!("{:?}", list.pop_back());
    println!("{:?}", list.pop_back());
    println!("{:?}", list.pop_back());
}
