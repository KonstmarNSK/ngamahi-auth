use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use futures::task::AtomicWaker;

mod in_memory;


pub mod user {
    use std::collections::HashSet;
    use crate::common_types::credentials::Password;
    use crate::common_types::{Login, Role};
    use crate::common_types::tokens::RefreshTokenHash;
    use crate::common_types::user_data::UserData;

    pub trait Storage {
        fn create_user_data(&mut self, data: UserData) -> Result<(), (UserData, StorageErr)>;
        fn load_user_data(&mut self, login: &Login) -> Result<Option<&UserData>, StorageErr>;

        // updates
        fn change_pass(&mut self, login: &Login, new_pass: Password, old_pass: Password)
            -> Result<(), StorageErr>;

        fn add_roles(&mut self, login: &Login, roles_to_add: HashSet<Role>) -> Result<(), (HashSet<Role>, StorageErr)>;
        fn remove_roles(&mut self, login: &Login, roles_to_remove: HashSet<Role>) -> Result<(), (HashSet<Role>, StorageErr)>;

        fn store(&mut self, token: RefreshTokenHash) -> Result<(), (RefreshTokenHash, StorageErr)>;

        /// Search given token in storage and delete it. Returns true if token was found, false otherwise
        fn delete(&mut self, token: RefreshTokenHash) -> Result<bool, (RefreshTokenHash, StorageErr)>;
    }

    pub enum StorageErr {
        LoginAlreadyRegistered,
        LoginNotFound,
    }
}

// Front storage sends a command to a back storage that owns the data.
// Return values are passed back to front by atomic references
pub mod storage_messages {
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
    use std::task::{Context, Poll};
    use futures::task::AtomicWaker;

    // request

    pub enum StorageReq {

    }



    // response

    #[derive(Clone)]
    pub struct StorageResp<TValue> {
        arc: Arc<StorageRespInner<TValue>>
    }

    struct StorageRespInner<TValue>{
        waker: AtomicWaker,
        set: AtomicBool,
        value: AtomicPtr<TValue>
    }

    impl<TValue> StorageResp<TValue> {
        pub fn new() -> Self {
            StorageResp{
                arc: Arc::new(
                    StorageRespInner {
                        waker: AtomicWaker::new(),
                        set: AtomicBool::new(false),
                        value: AtomicPtr::<TValue>::default()
                    }
                )
            }
        }

        pub fn set(self, mut val: TValue) {
            self.arc.value.store(&mut val, Ordering::Release);
            self.arc.set.store(true, Ordering::Release);    // todo: maybe Relaxed?
            self.arc.waker.wake();
        }
    }

    impl<TValue: Sized + Clone> Future for StorageResp<TValue> {
        type Output = Option<TValue>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // quick check to avoid registration if already done.
            if self.arc.set.load(Ordering::Acquire) {
                let val = self.arc.value.load(Ordering::Acquire);

                // todo: check safety
                return unsafe { Poll::Ready(val.as_ref().cloned()) };
            }

            self.arc.waker.register(cx.waker());

            // Need to check condition **after** `register` to avoid a race
            // condition that would result in lost notifications.
            if self.arc.set.load(Ordering::Acquire) {
                let val = self.arc.value.load(Ordering::Acquire);

                // todo: check safety
                return unsafe { Poll::Ready(val.as_ref().cloned()) };
            } else {
                Poll::Pending
            }
        }
    }
}

/// Front is immutable struct that holds Sender part of chanel to the thread that owns the data (back storage).
/// Front can create a StorageOperation object that gets a copy of Sender and sends a msg to the back storage.
/// The back storage sends return values back via atomic ptrs (it's encapsulated in StorageOperation)
pub mod front {
    use std::collections::HashSet;
    use std::sync::mpsc::Sender;
    use crate::common_types::credentials::Password;
    use crate::common_types::{Login, Role};
    use crate::common_types::tokens::RefreshTokenHash;
    use crate::common_types::user_data::UserData;
    use crate::storage::storage_messages::{StorageReq, StorageResp};
    use crate::storage::user::{Storage, StorageErr};

    pub struct Front {
        back_channel: Sender<StorageReq>
    }

    pub struct StorageOperation<Ret: Sized + Clone> {
        back_channel: Sender<StorageReq>,
        return_val: StorageResp<Ret>
    }

    impl <Ret: Sized + Clone> StorageOperation<Ret> {
        pub async fn await_result(self) -> Option<Ret> {
            self.return_val.await
        }
    }

    impl Front {
        pub fn storage<Ret: Sized + Clone>(&self) -> StorageOperation<Ret> {
            StorageOperation{
                back_channel: self.back_channel.clone(),
                return_val: StorageResp::<Ret>::new(),
            }
        }
    }

    impl<Ret: Sized + Clone> Storage for StorageOperation<Ret> {
        fn create_user_data(&mut self, data: UserData) -> Result<(), (UserData, StorageErr)> {
            todo!()
        }

        fn load_user_data(&mut self, login: &Login) -> Result<Option<&UserData>, StorageErr> {
            todo!()
        }

        fn change_pass(&mut self, login: &Login, new_pass: Password, old_pass: Password) -> Result<(), StorageErr> {
            todo!()
        }

        fn add_roles(&mut self, login: &Login, roles_to_add: HashSet<Role>) -> Result<(), (HashSet<Role>, StorageErr)> {
            todo!()
        }

        fn remove_roles(&mut self, login: &Login, roles_to_remove: HashSet<Role>) -> Result<(), (HashSet<Role>, StorageErr)> {
            todo!()
        }

        fn store(&mut self, token: RefreshTokenHash) -> Result<(), (RefreshTokenHash, StorageErr)> {
            todo!()
        }

        fn delete(&mut self, token: RefreshTokenHash) -> Result<bool, (RefreshTokenHash, StorageErr)> {
            todo!()
        }
    }
}