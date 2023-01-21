use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use password_hash::PasswordHashString;
use crate::common_types::tokens::{JWT, RefreshToken};
use crate::common_types::credentials::{Login, Password, Role, UserCredentials};
use crate::common_types::user_data::UserData;

mod in_memory;


pub trait Storage {
    fn create_user_data(&mut self, data: UserCreateData) -> Result<(), StorageErr>;

    fn update_user_data(&mut self, login: Login, data: UserUpdateData) -> Result<(), StorageErr>;

    fn load_user_data(&mut self, login: Login) -> Result<UserData, StorageErr>;
}

pub enum StorageErr{}


/// Data that will be updated.
#[derive(Clone)]
pub struct UserUpdateData {
    pass: Option<Password>,
    roles_to_add: Vec<Role>,
    roles_to_remove: Vec<Role>,
}

/// New user data
#[derive(Clone)]
pub struct UserCreateData {
    credentials: UserCredentials,
    roles: Vec<Role>,
}
