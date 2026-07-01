use std::{
    sync::{mpsc, Arc},
    thread,
};

fn main() {
    let loots = vec![10, 20, 30];
    let mut gold = 100;

    let (sender, receiver) = mpsc::sync_channel(3);
    let sender_arc = Arc::new(sender);

    for loot in loots.clone().into_iter() {
        thread::spawn({
            let thread_sender = Arc::clone(&sender_arc);
            move || {
                thread_sender.send(loot).unwrap();
            }
        });
    }

    for _ in 0..loots.len() {
        let loot = receiver.recv().unwrap();
        gold += loot;
    }

    println!("This is gold now : {}", gold);
}
