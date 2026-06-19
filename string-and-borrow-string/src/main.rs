fn main() {
    let map = String::from("Old map");

    let borrowed_map = map.as_str();

    let mut string_map = borrowed_map.to_string();

    string_map.push_str(" to new map");

    println!("{}", string_map);
}
