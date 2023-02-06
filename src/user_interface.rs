use crate::Database;
use crate::User;
use crate::input;

use rusqlite::Result;

/// UI implementation for registering a new user
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

/// UI implementation for logging in a user
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

/// UI implementation for showing all users
pub fn show_all_users(db: &Database) {
    let users = db.get_all_users().expect("Failed to get users");
    if users.len() == 0 {
        println!("No users available");
    }
    for user in users {
        println!("{:#?}", user);
    }
}

/// UI implementation for creating a product
pub fn create_product(db: &Database) {
    let name =input("Name: ").unwrap();
    let price = input("Price: ")
        .unwrap()
        .parse::<i32>()
        .unwrap();

    db.create_product(&name, price).unwrap();
}

/// UI implementation for showing all products
pub fn show_all_products(db: &Database)  {
    let products = db.get_all_products().unwrap();
    if products.len() == 0 {
        println!("No products available");
    }
    for product in products {
        println!("{:#?}", product);
    }
}