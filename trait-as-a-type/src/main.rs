trait Gear {
    fn use_gear(&self) {}
}

struct Sword;
struct Bow;
struct Potion;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("This is Sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("This is Bow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("This is Potion");
    }
}

fn use_gear<T: Gear>(item: T) {
    item.use_gear();
}

fn main() {
    let dave_sword = Sword;
    let dave_bow = Bow;
    let dave_potion = Potion;

    use_gear(dave_sword);
    use_gear(dave_bow);
    use_gear(dave_potion);
}
