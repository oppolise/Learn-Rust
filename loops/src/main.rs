fn main() {
    let treasures = ["Gold", "Silver", "Ruby Gem", "Emerald"];
    let mut energy = 3;

    for treasure in treasures.iter() {
        if energy == 0 {
            println!("You are lost your energy");
            break;
        } else if treasure == &"Ruby Gem" {
            println!("You found the Ruby Gem!");
            break;
        }

        energy -= 1;
    }
}
