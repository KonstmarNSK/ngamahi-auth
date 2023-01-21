mod in_memory;



pub mod user {
    use std::collections::HashSet;
    use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
    use password_hash::PasswordHashString;
    use crate::common_types::tokens::{JWT, RefreshToken};
    use crate::common_types::credentials::{Password, PasswordHash, UserCredentials};
    use crate::common_types::{Login, Role};
    use crate::common_types::user_data::UserData;



    pub trait UserStorage {
        fn create_user_data(&mut self, data: UserData) -> Result<(), (UserData, UserStorageErr)>;
        fn load_user_data(&mut self, login: &Login) -> Result<Option<&UserData>, UserStorageErr>;

        // updates
        fn change_pass(&mut self, login: &Login, new_pass: Password, old_pass: Password)
            -> Result<(), UserStorageErr>;

        fn add_roles(&mut self, login: &Login, roles_to_add: HashSet<Role>) -> Result<(), (HashSet<Role>, UserStorageErr)>;
        fn remove_roles(&mut self, login: &Login, roles_to_remove: HashSet<Role>) -> Result<(), (HashSet<Role>, UserStorageErr)>;
    }

    pub enum UserStorageErr {
        LoginAlreadyRegistered,
        LoginNotFound,
    }
}

pub mod refresh_token{
    use crate::common_types::tokens::{RefreshToken, RefreshTokenHash};


    pub trait RefreshTokenStorage{
        fn store(&mut self, token: RefreshTokenHash) -> Result<(), (RefreshTokenHash, RTStorageErr)>;

        /// Search given token in storage and delete it. Returns true if token was found, false otherwise
        fn delete(&mut self, token: RefreshTokenHash) -> Result<bool, (RefreshTokenHash, RTStorageErr)>;
    }

    pub enum RTStorageErr{}
}