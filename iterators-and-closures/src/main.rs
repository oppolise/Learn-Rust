fn main() {
    let treasure = vec![100, 200, 300, 400];
    let double: Vec<i32> = treasure.iter().map(|x| x * 2).collect();
    println!("{:?}", double);
}
