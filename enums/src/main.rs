enum Daveza {
    Fighting,
    Collecting(u32),
    Defending,
}

impl Daveza {
    fn state_represent(&self) {
        match self {
            Daveza::Fighting => println!("Dave Fighting"),
            Daveza::Defending => println!("Dave Defending"),
            Daveza::Collecting(result) => println!("Dave is Collecting: {}", result),
        }
    }
}

fn main() {
    let dave_fighting = Daveza::Fighting;
    let dave_defending = Daveza::Defending;
    let dave_collecting = Daveza::Collecting(10);

    dave_fighting.state_represent();
    dave_defending.state_represent();
    dave_collecting.state_represent();
}
