use std::sync::{mpsc, Arc};

fn main() {
    let loots = vec![10, 20, 30];
    let mut gold = 100;

    let (sender, receiver) = mpsc::sync_channel(3);
    let sender_arc = Arc::new(sender);
}
