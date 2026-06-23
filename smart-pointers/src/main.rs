use std::{cell::RefCell, rc::Rc};

fn main() {
    let chest = Box::new(10);
    let share_chest = Rc::new(RefCell::new(chest));

    println!("{:?}", *share_chest);
    println!("{:?}", **share_chest.borrow());

    **share_chest.borrow_mut() += 100;
    println!("{:?}", share_chest.borrow());
}
