use std::{eprintln, io, println};
use tokio::sync::mpsc::channel;

struct Book {
    title: &'static str,
}

impl Book {
    fn new(title: &'static str) -> Self {
        Self { title }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let (book_sender, mut book_receiver) = channel(5);

    // Each store gets its own copy of the sender.
    let store_one_book_sender = book_sender.clone();
    let book_store_one = tokio::task::spawn(async move {
        if let Err(err) = store_one_book_sender
            .send(Book::new("Shawshank Redemption"))
            .await
        {
            eprintln!("Failed to send book from store one: {}", err)
        }
    });

    let book_store_two = tokio::task::spawn(async move {
        if let Err(err) = book_sender.send(Book::new("Secret Recipe")).await {
            eprintln!("Failed to send book from store two: {}", err);
        }
    });

    let mut shelf: Vec<Book> = Vec::new();
    while let Some(new_book) = book_receiver.recv().await {
        shelf.push(new_book);
    }

    book_store_one.await?;
    book_store_two.await?;

    for book in &shelf {
        println!("Title: {}", book.title);
    }

    Ok(())
}
