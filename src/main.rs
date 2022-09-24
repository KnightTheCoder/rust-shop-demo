mod database;
mod utilities;
mod user;

use database::Database;
use utilities::{input, clear_screen};
use user::User;

use rusqlite::Result;
use std::process;

fn main() {
    let db = Database::new("users.db").expect("Error creating database");
    db.create_users_table().expect("Error creating users table");
    let mut user = User::default();
    
    loop {
        clear_screen();

        println!("Register (0)");
        println!("Login (1)");
        println!("Show all users (2)");
        println!("Exit (3)");
        
        let option = input(">")
        .expect("Failed to get input")
        .parse::<i32>()
        .expect("Failed to parse");
        
        clear_screen();

        match option {
            0 => {
                user = match register(&db) {
                    Ok(res) => res,
                    Err(err) => {
                        println!("{}", err.to_string());
                        User::default()
                    }
                };
            }
            1 => {
                login(&db, &user);
            }
            2 => {
                let users = db.get_all_users().expect("Failed to get users");
                if users.len() == 0 {
                    println!("No users available");
                }
                for user in users {
                    println!("{:#?}", user);
                }
            }
            3 => process::exit(0),
            _ => ()
        }
        input("Press any key to continue").expect("Failed to get input");
    }
    
}

fn register(db: &Database) -> Result<User> {
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

fn login(db: &Database, user: &User) {
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