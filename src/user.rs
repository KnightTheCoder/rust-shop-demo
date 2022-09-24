use rusqlite::Result;

use crate::{
    utilities::hash_password,
    database::Database
};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    password: String
}

impl User {
    pub fn new(id: i32, username: &str, password: &str) -> Self {
        Self {
            id,
            username: username.to_string(),
            password: hash_password(password)
        }
    }

    pub fn password(&self) -> String {
        self.password.clone()
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.password = hash_password(new_password);
    }

    pub fn register(&self, db: &Database) -> Result<bool> {
        let affected_rows = db.create_user(&self.username, &self.password)?;
        if affected_rows > 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn login(&self, db: &Database, username: &str, password: &str) -> Result<bool> {
        let password = &hash_password(password);
        let user = db.get_user(username, password)?;
        if user.id != -1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: -1,
            username: String::new(),
            password: String::new()
        }
    }
}