use std::io::{self, stdout, Write, stdin};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

/// Clears the screen
pub fn clear_screen() {
    print!("{}c", 27 as char);
}

/// Displays a message and gets the input
pub fn input(msg: &str) -> io::Result<String> {
    print!("{}", msg);
    stdout().flush()?;

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    
    let buffer = buffer.trim().to_string();
    Ok(buffer)
}

/// Hashes the given password using [`Sha256`] hashing
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(password);
    hasher.result_str()
}