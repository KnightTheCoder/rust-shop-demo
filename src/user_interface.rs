use crate::database::Database;
use crate::user::User;
use crate::utilities::input;

use rusqlite::Result;

pub fn register(db: &Database) -> Result<User> {
    println!("Register user");
    let username = input("Username: ").unwrap();
    let password = input("Passowrd: ").unwrap();
    let user = User::new(-1, &username, &password);
    
    match user.register(&db) {
        Ok(res) => {
            if res {
                println!("Successfully registered new account");
            } else {
                println!("Unsuccessfully registered new account");
            }
            Ok(user)
        }
        Err(err) => Err(err)
    }
}

pub fn login(db: &Database, user: &User) {
    println!("Login user");
    let username = input("Username: ").unwrap();
    let password = input("Password: ").unwrap();
    match user.login(&db, &username, &password) {
        Ok(res) => {
            if res  {
                println!("Successfull login!");
            } else {
                println!("Failed to login!");
                println!("Username or Password is incorrect!")
            }
        }
        Err(err) => eprintln!("{}", err.to_string())
    }
}