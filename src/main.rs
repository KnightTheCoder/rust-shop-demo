mod models;
mod database;
mod utilities;
mod user_interface;

use database::Database;
use models::User;
use utilities::{
    input,
    hash_password,
    clear_screen
};
use user_interface::{
    register,
    login,
    show_all_users,
    create_product,
    show_all_products
};
use std::process;

fn main() {
    let mut user = User::default();
    
    loop {
        let db = Database::new("users.db").expect("Error creating database");
        db.create_tables().expect("Error creating users table");

        clear_screen();

        println!("Register (0)");
        println!("Login (1)");
        println!("Create product (2)");
        println!("Show all users (3)");
        println!("Show all products (4)");
        println!("Exit (5)");
        
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
            2 => create_product(&db),
            3 => show_all_users(&db),
            4 => show_all_products(&db),
            5 => process::exit(0),
            _ => ()
        }
        input("Press any key to continue").expect("Failed to get input");
    }
    
}