fn main() {
    let x = 20;
    let y = 30;

    let result = sum(x, y);
    println!("Result of sum is {}", result)
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
