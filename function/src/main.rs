fn main() {
    let x = 10;
    let y = 20;

    let result = sum(x, y);
    println!("Result of sum is {}", result)
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
