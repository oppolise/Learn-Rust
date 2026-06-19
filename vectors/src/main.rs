fn main() {
    let mut items = vec!["Gold", "Silver", "Ruby Gem", "Emerald"];
    items.push("dave");
    println!("items: {:?}", items);
    println!("items length: {}", items.len());
    println!("items capacity: {}", items.capacity());

    items.pop();
    println!("items: {:?}", items);
    println!("items length: {}", items.len());
    println!("items capacity: {}", items.capacity());

    items.remove(1);
    println!("items: {:?}", items);
    println!("items length: {}", items.len());
    println!("items capacity: {}", items.capacity());
}
