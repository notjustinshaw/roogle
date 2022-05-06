use core::cell::RefCell;

fn main() {
    let c = RefCell::new("hello".to_owned());

    *c.borrow_mut() = "bonjour".to_owned();

    assert_eq!(&*c.borrow(), "bonjour");
}
