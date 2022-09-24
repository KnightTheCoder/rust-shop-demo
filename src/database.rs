use rusqlite::{Connection, Result};
use crate::user::User;

#[derive(Debug)]
pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new(fname: &str) -> Result<Self> {
        Ok(
            Self {
                conn: Connection::open(fname)?
            }
        )
    }

    pub fn from_memory() -> Result<Self> {
        Ok(
            Self {
                conn: Connection::open_in_memory()?
            }
        )
    }

    pub fn create_users_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                password TEXT NOT NULL
            );", []
        )?;

        Ok(())
    }

    pub fn create_user(&self, username: &str, password: &str) -> Result<usize> {
        let result = self.conn.execute(
        "INSERT INTO users (username, password) VALUES (?, ?)
        ;",&[username, password]);

        match result {
            Ok(affected_rows) => Ok(affected_rows),
            Err(err) => Err(err)
        }
    }

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
}