use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let gold = Arc::new(Mutex::new(10));

    let loot_1 = thread::spawn({
        let gold_artifact = Arc::clone(&gold);
        move || {
            let mut gold = gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    let loot_2 = thread::spawn({
        let gold_artifact = Arc::clone(&gold);
        move || {
            let mut gold = gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    let loot_3 = thread::spawn({
        let gold_artifact = Arc::clone(&gold);
        move || {
            let mut gold = gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    loot_1.join().unwrap();
    loot_2.join().unwrap();
    loot_3.join().unwrap();

    println!("Gold: {}", gold.lock().unwrap());
}
