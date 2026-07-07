use age::Recipient;
use std::io::Read;

fn main() {
    // Check Encryptor::with_recipients return type
    let recipients: Vec<Box<dyn Recipient + Send>> = vec![];
    let encryptor = age::Encryptor::with_recipients(recipients);
    match encryptor {
        Some(_) => println!("with_recipients returns Some"),
        None => println!("with_recipients returns None for empty list"),
    }

    // Check Decryptor variants (this is a compile-time check mostly)
}
