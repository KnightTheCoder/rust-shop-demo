use rusqlite::{Connection, Result};
use crate::{User, models::Product};

/// [`Database`] is a collection of methods 
/// made to work with an SQlite [`Connection`]
#[derive(Debug)]
pub struct Database {
    conn: Connection
}

impl Database {
    /// Creates a new [`Database`] with an SQlite connection 
    /// to the given file
    pub fn new(fname: &str) -> Result<Self> {
        Ok(
            Self {
                conn: Connection::open(fname)?
            }
        )
    }

    /// Creates a new [`Database`] with an SQlite [`Connection`] 
    /// in memory
    pub fn open_in_memory() -> Result<Self> {
        Ok(
            Self {
                conn: Connection::open_in_memory()?
            }
        )
    }

    /// Creates tables if they don't already exist
    pub fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS products(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                price INTEGER NOT NULL
            );", []
        )?;

        Ok(())
    }

    /// Creates a new user and 
    /// returns whether the operation succeeded
    pub fn create_user(&self, username: &str, password: &str) -> Result<bool> {
        if username.trim().len() == 0 && password.trim().len() == 0 {
            return Ok(false)
        }

        let result = self.conn.execute(
        "INSERT INTO users (username, password) VALUES (?, ?)
        ;",&[username, password]);

        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(err)
        }
    }

    /// Gets a user from the [`Database`]
    pub fn get_user(&self, username: &str, password: &str) -> Result<User> {
        let mut user = User::default();

        let mut stmt = self.conn.prepare(
            "SELECT id, username, password FROM users
            WHERE username = :username AND password = :password
        ;")?;

        let user_iter = stmt.query_map(
            &[(":username", username), (":password", password)],
            |row| {
                let username: String = row.get(1)?;
                let password: String = row.get(2)?;
                Ok(
                    User::new(
                        row.get(0)?,
                        &username,
                        &password
                    )
                )
            }
        )?;

        for usr in user_iter {
            user =usr?;
        }

        Ok(user)
    }

    /// Gets all users from the [`Database`]
    pub fn get_all_users(&self) -> Result<Vec<User>> {
        let mut users = vec![];
        let mut stmt = self.conn.prepare(
            "SELECT id, username, password FROM users"
        )?;

        let user_iter = stmt.query_map([], |row| {
            let username: String = row.get(1)?;
            let password: String = row.get(2)?;
            Ok(
                User::new(
                    row.get(0)?,
                    &username,
                    &password
                )
            )
        })?;

        for user in user_iter {
            users.push(user?);
        }

        Ok(users)
    }

    pub fn create_product(&self, name: &str, price: i32) -> Result<bool> {
        if name.trim().len() == 0 || price < 0 {
            return Ok(false);
        }

        let result = self.conn.execute(
            "INSERT INTO products (name, price) VALUES (?, ?);",
            (name, price));

        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(err)
        }
    }

    pub fn get_product(&self, name: &str) -> Result<Product> {
        let mut product = Product::default();

        let mut stmt = self.conn.prepare(
            "SELECT id, name, price FROM products WHERE name = :name;"
        )?;

        let product_iter = stmt.query_map(
            &[(":name", name)], |row| {
            let name: String = row.get(1)?;
            Ok(
                Product::new(
                    row.get(0)?,
                    &name,
                    row.get(2)?
                )
            )
        })?;

        for prdt in product_iter {
            product = prdt?;
        }

        Ok(product)
    }

    pub fn get_all_products(&self) -> Result<Vec<Product>> {
        let mut products = vec![];

        let mut stmt = self.conn.prepare(
            "SELECT id, name, price FROM products;"
        )?;

        let product_iter = stmt.query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(
                Product::new(
                    row.get(0)?,
                    &name,
                    row.get(2)?
                )
            )
        })?;

        for prdt in product_iter {
            products.push(prdt?);
        }

        Ok(products)
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use rusqlite::Result;

    fn setup() -> Result<Database> {
        let db = Database::open_in_memory()?;
        db.create_tables()?;
        Ok(db)
    }

    #[test]
    fn create_database() {
        let result = match setup() {
            Ok(_) => true,
            Err(_) => false
        };
        assert_eq!(result, true)
    }

    #[test]
    fn create_user_success() {
        let db = setup().unwrap();
        let success = db.create_user("User", "123").expect("Error creating user");
        
        assert_eq!(success, true)
    }

    #[test]
    fn create_user_failure() {
        let db = setup().unwrap();
        let success = db.create_user("", "").expect("Error creating user");
        
        assert_eq!(success, false)
    }

    #[test]
    fn get_created_user_success() {
        let username = "new_user";
        let password = "password";

        let db = setup().unwrap();
        db.create_user(username, password).unwrap();

        let user = db.get_user(username, password).unwrap();
        assert_ne!(user.id, -1)
    }

    #[test]
    fn get_created_user_with_empty_credentials() {
        let username = "";
        let password = "";

        let db = setup().unwrap();
        db.create_user(username, password).unwrap();

        let user = db.get_user(username, password).unwrap();
        assert_eq!(user.id, -1)
    }

    #[test]
    fn create_multiple_users() {
        let db = setup().unwrap();
        for i in 0..5 {
            let i = &i.to_string();
            db.create_user(&(String::from("user") + i), "password").unwrap();
        }
        let users = db.get_all_users().unwrap();
        assert_eq!(users.len(), 5)
    }
}