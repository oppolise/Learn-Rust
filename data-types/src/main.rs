fn main() {
    let int1 = 1;
    let float1 = 0.5;

    let result_int = int1 + (float1 as i32);
    let result_float = (int1 as f32) + float1;

    let mut msg = String::from("Hello, World!");
    let msg2 = "can't push string int type &str";
    let mut msg3 = "Hello, World!".to_string();
    let mut msg4 = format!("Point: {}, {}, {}", int1, float1, result_int);

    msg += "2";
    msg3 += "2";
    msg4 += "2";

    println!("result_int: {}", result_int);
    println!("result_float: {}", result_float);
    println!("msg: {}", msg);
    println!("msg2: {}", msg2);
    println!("msg3: {}", msg3);
    println!("msg4: {}", msg4);
}
