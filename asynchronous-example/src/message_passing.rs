use std::{io, println};
use tokio::sync::mpsc::unbounded_channel;

struct Message {
    from: &'static str,
    text: &'static str,
}

impl Message {
    fn new(from: &'static str, text: &'static str) -> Self {
        Self { from, text }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let (message_sender, mut message_receiver) = unbounded_channel();

    let alice_message_sender = message_sender.clone();
    let person_one = tokio::task::spawn(async move {
        if let Err(err) = alice_message_sender.send(Message::new("Alice", "Meeting postponed")) {
            eprintln!("Failed to send message from Alice: {}", err);
        }
    });

    let person_two = tokio::task::spawn(async move {
        if let Err(err) = message_sender.send(Message::new("Bob", "Secret Leaked")) {
            eprintln!("Failed to send message from Bob: {}", err);
        }
    });

    let mut inbox: Vec<Message> = Vec::new();
    while let Some(new_book) = message_receiver.recv().await {
        inbox.push(new_book);
    }

    person_one.await?;
    person_two.await?;

    for msg in &inbox {
        println!("{} says: {}", msg.from, msg.text);
    }

    Ok(())
}
