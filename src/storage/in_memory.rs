use std::collections::HashMap;
use crate::common_types::Login;
use crate::common_types::user_data::UserData;
use crate::storage::refresh_token::RefreshTokenStorage;
use crate::storage::user::{UserStorage, UserStorageErr};



/// A toy storage that holds all the data in local RAM
pub mod user_storage {
    use std::collections::{HashMap, HashSet};
    use crate::common_types::credentials::{Password, PasswordHash};
    use crate::common_types::{Login, Role};
    use crate::common_types::user_data::UserData;
    use crate::storage::user::{UserStorage, UserStorageErr};

    pub struct Storage {
        users_data: HashMap<Login, UserData>
    }

    impl UserStorage for Storage {
        fn create_user_data(&mut self, data: UserData) -> Result<(), (UserData, UserStorageErr)> {

            if self.users_data.contains_key(&data.credentials.login) {
                return Err((data, UserStorageErr::LoginAlreadyRegistered));
            };

            self.users_data.insert(data.credentials.login.clone(), data);
            Ok(())
        }

        fn load_user_data(&mut self, login: &Login) -> Result<Option<&UserData>, UserStorageErr> {
            Ok(self.users_data.get(login))
        }

        fn change_pass(&mut self, login: &Login, new_pass: Password, old_pass: Password) -> Result<(), UserStorageErr> {
            let user_data = self.users_data.get_mut(login).ok_or_else(|| UserStorageErr::LoginNotFound)?;

            if &user_data.credentials.pass == &old_pass {
                user_data.credentials.pass = new_pass;
            };

            Ok(())
        }

        fn add_roles(&mut self, login: &Login, mut roles_to_add: HashSet<Role>) -> Result<(), (HashSet<Role>, UserStorageErr)> {

            let user_data = match self.users_data.get_mut(login) {
                Some(data) => data,
                _ => return Err((roles_to_add, UserStorageErr::LoginNotFound)),
            };

            user_data.roles.extend(&mut roles_to_add.into_iter());

            Ok(())
        }

        fn remove_roles(&mut self, login: &Login, roles_to_remove: HashSet<Role>) -> Result<(), (HashSet<Role>, UserStorageErr)> {

            let mut user_data = match self.users_data.get_mut(login) {
                Some(data) => data,
                _ => return Err((roles_to_remove, UserStorageErr::LoginNotFound)),
            };

            user_data.roles.retain(|role| !roles_to_remove.contains(role));

            Ok(())
        }
    }
}



pub mod rt_storage {
    use std::collections::{HashMap, HashSet};
    use crate::common_types::Login;
    use crate::common_types::tokens::RefreshTokenHash;
    use crate::common_types::user_data::UserData;
    use crate::storage::refresh_token::{RefreshTokenStorage, RTStorageErr};

    pub struct Storage {
        tokens: HashSet<RefreshTokenHash>
    }

    impl RefreshTokenStorage for Storage {
        fn store(&mut self, token: RefreshTokenHash) -> Result<(), (RefreshTokenHash, RTStorageErr)> {
            self.tokens.insert(token);

            Ok(())
        }

        fn delete(&mut self, token: RefreshTokenHash) -> Result<bool, (RefreshTokenHash, RTStorageErr)> {
            Ok(self.tokens.remove(&token))
        }
    }
}
