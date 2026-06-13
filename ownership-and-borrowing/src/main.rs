fn main() {
    let mut treasure = String::from("gold coins");

    let frined1 = &treasure;
    let frined2 = &treasure;
    println!("Frined 1 sees: {}", frined1);
    println!("Frined 2 sees: {}", frined2);

    let trusted_frined = &mut treasure;
    trusted_frined.push_str(" add silver coins");
    println!("Trusted Frined update: {}", trusted_frined);
}
