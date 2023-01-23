use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use actix::{Actor, Context, Handler};
use crate::common_types::Role;
use crate::common_types::tokens::RefreshTokenHash;

use crate::common_types::user_data::UserData;
use crate::storage::storage_messages::{AddRoles, ChangePass, CreateUserData, DeleteRT, LoadUserData, RemoveRoles, StoreRT};
use crate::storage::user::{Storage, StorageErr};

mod in_memory;


pub mod user {
    use std::collections::HashSet;
    use crate::common_types::credentials::Password;
    use crate::common_types::{Login, Role};
    use crate::common_types::tokens::RefreshTokenHash;
    use crate::common_types::user_data::UserData;

    pub trait Storage {
        fn create_user_data(&mut self, data: UserData) -> Result<(), (UserData, StorageErr)>;
        fn load_user_data(&mut self, login: &Login) -> Result<Option<UserData>, StorageErr>;

        // updates
        fn change_pass(&mut self, login: &Login, new_pass: Password, old_pass: Password)
            -> Result<(), StorageErr>;

        fn add_roles(&mut self, login: &Login, roles_to_add: HashSet<Role>) -> Result<(), (HashSet<Role>, StorageErr)>;
        fn remove_roles(&mut self, login: &Login, roles_to_remove: HashSet<Role>) -> Result<(), (HashSet<Role>, StorageErr)>;

        fn store_rt(&mut self, token: RefreshTokenHash) -> Result<(), (RefreshTokenHash, StorageErr)>;

        /// Search given token in storage and delete it. Returns true if token was found, false otherwise
        fn delete_rt(&mut self, token: RefreshTokenHash) -> Result<bool, (RefreshTokenHash, StorageErr)>;
    }

    pub enum StorageErr {
        LoginAlreadyRegistered,
        LoginNotFound,
    }
}

// Front storage sends a command to a back storage that owns the data.
// Return values are passed back to front by atomic references
pub mod storage_messages {
    use std::collections::HashSet;
    use actix::Message;
    use crate::common_types::credentials::Password;
    use crate::common_types::{Login, Role};
    use crate::common_types::tokens::RefreshTokenHash;
    use crate::common_types::user_data::UserData;
    use crate::storage::user::StorageErr;

    // request
    pub struct CreateUserData{ pub data: UserData }
    pub struct LoadUserData{ pub login: Login }
    pub struct ChangePass{ pub login: Login, pub new_pass: Password, pub old_pass: Password }
    pub struct AddRoles{ pub login: Login, pub roles_to_add: HashSet<Role> }
    pub struct RemoveRoles{ pub login: Login, pub roles_to_remove: HashSet<Role> }
    pub struct StoreRT{ pub token: RefreshTokenHash }
    pub struct DeleteRT{ pub token: RefreshTokenHash }

    impl Message for CreateUserData { type Result = Result<(), (UserData, StorageErr)>; }
    impl Message for LoadUserData { type Result = Result<Option<UserData>, StorageErr>; }
    impl Message for ChangePass { type Result = Result<(), StorageErr>; }
    impl Message for AddRoles { type Result = Result<(), (HashSet<Role>, StorageErr)>; }
    impl Message for RemoveRoles { type Result = Result<(), (HashSet<Role>, StorageErr)>; }
    impl Message for StoreRT { type Result = Result<(), (RefreshTokenHash, StorageErr)>; }
    impl Message for DeleteRT { type Result = Result<bool, (RefreshTokenHash, StorageErr)>; }
}

pub enum InnerStorage{
    InMemoryLocal(in_memory::user_storage::Storage),
}

pub struct StorageActor {
    storage: InnerStorage,
}

impl Actor for StorageActor {
    type Context = Context<Self>;
}


impl Handler<CreateUserData> for StorageActor {
    type Result = Result<(), (UserData, StorageErr)>;

    fn handle(&mut self, msg: CreateUserData, ctx: &mut Self::Context) -> Self::Result {
        match &mut self.storage {
            InnerStorage::InMemoryLocal(storage) => storage.create_user_data(msg.data)
        }
    }
}

impl Handler<LoadUserData> for StorageActor {
    type Result = Result<Option<UserData>, StorageErr>;

    fn handle(&mut self, msg: LoadUserData, ctx: &mut Self::Context) -> Self::Result {
        match &mut self.storage {
            InnerStorage::InMemoryLocal(storage) => storage.load_user_data(&msg.login)
        }
    }
}

impl Handler<ChangePass> for StorageActor {
    type Result = Result<(), StorageErr>;

    fn handle(&mut self, msg: ChangePass, ctx: &mut Self::Context) -> Self::Result {
        match &mut self.storage {
            InnerStorage::InMemoryLocal(storage) => storage.change_pass(&msg.login, msg.new_pass, msg.old_pass)
        }
    }
}

impl Handler<AddRoles> for StorageActor {
    type Result = Result<(), (HashSet<Role>, StorageErr)>;

    fn handle(&mut self, msg: AddRoles, ctx: &mut Self::Context) -> Self::Result {
        match &mut self.storage {
            InnerStorage::InMemoryLocal(storage) => storage.add_roles(&msg.login, msg.roles_to_add)
        }
    }
}

impl Handler<RemoveRoles> for StorageActor {
    type Result = Result<(), (HashSet<Role>, StorageErr)>;

    fn handle(&mut self, msg: RemoveRoles, ctx: &mut Self::Context) -> Self::Result {
        match &mut self.storage {
            InnerStorage::InMemoryLocal(storage) => storage.remove_roles(&msg.login, msg.roles_to_remove)
        }
    }
}

impl Handler<StoreRT> for StorageActor {
    type Result = Result<(), (RefreshTokenHash, StorageErr)>;

    fn handle(&mut self, msg: StoreRT, ctx: &mut Self::Context) -> Self::Result {
        match &mut self.storage {
            InnerStorage::InMemoryLocal(storage) => storage.store_rt(msg.token)
        }
    }
}

impl Handler<DeleteRT> for StorageActor {
    type Result = Result<bool, (RefreshTokenHash, StorageErr)>;

    fn handle(&mut self, msg: DeleteRT, ctx: &mut Self::Context) -> Self::Result {
        match &mut self.storage {
            InnerStorage::InMemoryLocal(storage) => storage.delete_rt(msg.token)
        }
    }
}