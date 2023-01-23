
/// A toy storage that holds all the data in local RAM
pub mod user_storage {
    use std::collections::{HashMap, HashSet};
    use crate::common_types::credentials::{Password, PasswordHash};
    use crate::common_types::{Login, Role};
    use crate::common_types::tokens::RefreshTokenHash;
    use crate::common_types::user_data::UserData;
    use crate::storage::user::{Storage as UserStorage, StorageErr};

    pub struct Storage {
        users_data: HashMap<Login, UserData>,
        tokens: HashSet<RefreshTokenHash>,
    }

    impl UserStorage for Storage {
        fn create_user_data(&mut self, data: UserData) -> Result<(), (UserData, StorageErr)> {

            if self.users_data.contains_key(&data.credentials.login) {
                return Err((data, StorageErr::LoginAlreadyRegistered));
            };

            self.users_data.insert(data.credentials.login.clone(), data);
            Ok(())
        }

        fn load_user_data(&mut self, login: &Login) -> Result<Option<&UserData>, StorageErr> {
            Ok(self.users_data.get(login))
        }

        fn change_pass(&mut self, login: &Login, new_pass: Password, old_pass: Password) -> Result<(), StorageErr> {
            let user_data = self.users_data.get_mut(login).ok_or_else(|| StorageErr::LoginNotFound)?;

            if &user_data.credentials.pass == &old_pass {
                user_data.credentials.pass = new_pass;
            };

            Ok(())
        }

        fn add_roles(&mut self, login: &Login, mut roles_to_add: HashSet<Role>) -> Result<(), (HashSet<Role>, StorageErr)> {

            let user_data = match self.users_data.get_mut(login) {
                Some(data) => data,
                _ => return Err((roles_to_add, StorageErr::LoginNotFound)),
            };

            user_data.roles.extend(&mut roles_to_add.into_iter());

            Ok(())
        }

        fn remove_roles(&mut self, login: &Login, roles_to_remove: HashSet<Role>) -> Result<(), (HashSet<Role>, StorageErr)> {

            let mut user_data = match self.users_data.get_mut(login) {
                Some(data) => data,
                _ => return Err((roles_to_remove, StorageErr::LoginNotFound)),
            };

            user_data.roles.retain(|role| !roles_to_remove.contains(role));

            Ok(())
        }

        fn store_rt(&mut self, token: RefreshTokenHash) -> Result<(), (RefreshTokenHash, StorageErr)> {
            self.tokens.insert(token);
            Ok(())
        }

        fn delete_rt(&mut self, token: RefreshTokenHash) -> Result<bool, (RefreshTokenHash, StorageErr)> {
            Ok(self.tokens.remove(&token))
        }
    }
}


