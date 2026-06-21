use std::collections::HashMap;

fn main() {
    let mut treasure = HashMap::new();
    treasure.insert("Gem", 100);
    treasure.insert("Gold", 200);

    if let Some(money) = treasure.get_mut("Gold") {
        *money *= 2;
    }

    println!("{:?}", treasure);
}
