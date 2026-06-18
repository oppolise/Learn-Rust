struct Inventory<T> {
    item: T,
}

trait DisplayItem {
    fn display(&self);
}

impl<T> DisplayItem for Inventory<T>
where
    T: std::fmt::Debug,
{
    fn display(&self) {
        println!("This Inventory is {:?}", self.item);
    }
}

fn main() {
    let inventory_string = Inventory {
        item: "Hello World",
    };
    inventory_string.display();

    let inventory_int = Inventory { item: 100 };
    inventory_int.display();
}
