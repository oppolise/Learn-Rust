fn main() {
    println!("This is if-else");
    let x_if = 50;

    if x_if >= 70 {
        println!("this is x_if more than 70");
    } else {
        println!("this is x_if less than 70");
    }

    println!("This is match");
    let x_match = "&str";

    match x_match {
        "&str" => println!("Data Tpye x_match is &str"),
        _ => println!("We loss it.."),
    }

    println!("This is Loop");
    let mut x_loop = 0;

    loop {
        x_loop += 1;
        println!("Value of x_loop is {}", x_loop);

        if x_loop == 5 {
            break;
        }
    }

    println!("This is array control flow");
    let array = [40, 50, 70];

    for index in array {
        println!("Value in array is {}", index);
    }
}
