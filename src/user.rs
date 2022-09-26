use rusqlite::Result;

use crate::{
    utilities::hash_password,
    database::Database
};

/// Represents a user
/// contains information about the user
/// and it's used to interact with a [`Database`]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    password: String
}

impl User {
    /// Creates a [`User`] with a hashed password
    pub fn new(id: i32, username: &str, password: &str) -> Self {
        Self {
            id,
            username: username.to_string(),
            password: hash_password(password)
        }
    }

    /// Gets the value of the user's hashed password
    pub fn password(&self) -> String {
        self.password.clone()
    }

    /// Sets and hashes a new password for the user
    pub fn set_password(&mut self, new_password: &str) {
        self.password = hash_password(new_password);
    }

    /// Registers a new [`User`] and 
    /// returns whether the operation succeeded
    pub fn register(&self, db: &Database) -> Result<bool> {
        let success = db.create_user(&self.username, &self.password)?;
        Ok(success)
    }

    /// Logs in a [`User`] and 
    /// returns whether the operation succeeded
    pub fn login(&self, db: &Database, username: &str, password: &str) -> Result<bool> {
        let user = db.get_user(username, &hash_password(password))?;
        Ok(user.id != -1)
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

#[cfg(test)]
mod tests {
    use super::User;
    use crate::{
        database::Database,
        utilities::hash_password
    };

    #[test]
    fn user_password_encryption() {
        let hash = hash_password("password");
        let mut user = User::default();
        user.set_password("password");
        
        assert_eq!(user.password(), hash)
    }

    #[test]
    fn register_success() {
        let db = Database::open_in_memory().unwrap();
        db.create_users_table().unwrap();

        let user = User::new(-1, "user", "password");
        let success = user.register(&db).unwrap();

        assert_eq!(success, true)
    }

    #[test]
    fn register_failure() {
        let db = Database::open_in_memory().unwrap();
        db.create_users_table().unwrap();

        let user = User::default();
        let success = user.register(&db).unwrap();

        assert_eq!(success, false)
    }

    #[test]
    fn login_success() {
        let db = Database::open_in_memory().unwrap();
        db.create_users_table().unwrap();

        let user = User::new(-1, "user", "password");
        let success = user.register(&db).unwrap();
        assert_eq!(success, true);

        let success = user.login(&db, "user", "password").unwrap();
        assert_eq!(success, true)
    }

    #[test]
    fn login_failure() {
        let db = Database::open_in_memory().unwrap();
        db.create_users_table().unwrap();

        let user = User::default();
        user.register(&db).unwrap();

        let success = user.login(&db, "", "").unwrap();
        assert_eq!(success, false)
    }
}