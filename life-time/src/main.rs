fn main() {
    let treasure;
    {
        let local_treasure = String::from("Gold coins");
        treasure = &local_treasure;
        println!("{}", treasure)
    }
    // println!("{}", treasure)
}
