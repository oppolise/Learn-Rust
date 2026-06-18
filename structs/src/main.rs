struct Daveza {
    name: String,
    health: u8,
    age: u8,
}

impl Daveza {
    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal >= 100 {
            self.health = 100;
            return;
        }
        self.health += heal;
    }
}

fn main() {
    let mut dave = Daveza {
        name: "dave".to_string(),
        health: 100,
        age: 20,
    };
    dave.take_damage(40);
    println!("{} take damage", dave.name);
    println!("{} health is {}", dave.name, dave.health);

    dave.healing(40);
    println!("{} health is {}", dave.name, dave.health);
    println!("{} age is {}", dave.name, dave.age);
}
