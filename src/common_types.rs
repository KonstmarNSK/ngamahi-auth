

pub type Login = String;    // must be unique
pub type Role = String;
pub type Salt = String;
pub type Pepper = String;


pub mod tokens {
    use argon2::{Algorithm, Argon2, Params, Version};
    use password_hash::PasswordHasher;
    use serde::Deserialize;
    use crate::common_types::{Salt, Pepper};

    pub struct JWT {
        pub header: Header,
        pub payload: Payload,
        pub signature: Signature,
    }

    pub struct Header {}

    pub struct Payload {}

    pub struct Signature {}


    #[derive(Deserialize)]
    pub struct RefreshToken { token: String }


    /// Storing refresh token alongside with password hash is like
    /// storing the password itself in plaintext alongside its hash.
    #[derive(Clone)]
    #[derive(PartialEq, Eq, Hash)]
    pub struct RefreshTokenHash(String);

    impl RefreshTokenHash {
        pub fn new(salt: &Salt, pepper: &Pepper, token: &RefreshToken) -> Self {
            let hashing_algorithm = Algorithm::default();

            let ctx = Argon2::new_with_secret(
                pepper.as_bytes(),
                hashing_algorithm,
                Version::default(),
                Params::default(),
            ).unwrap();

            RefreshTokenHash(
                ctx.hash_password(token.token.as_bytes(), &salt).unwrap().serialize().to_string()
            )
        }
    }
}


pub mod credentials {
    use argon2::{Algorithm, Argon2, Params, Version};
    use password_hash::PasswordHasher;
    use crate::common_types::{Login, Pepper, Salt};


    #[derive(Clone)]
    pub struct UserCredentials {
        pub login: Login,
        pub pass: Password,
    }

    #[derive(Clone)]
    #[derive(Eq, PartialEq)]
    pub struct Password {
        pub salt: Salt,
        pub hash: PasswordHash,
    }


    // fixme: compare via password_hash crate
    #[derive(Clone)]
    #[derive(PartialEq, Eq, Hash)]
    pub struct PasswordHash(String);

    impl PasswordHash {
        pub fn new(salt: &Salt, pepper: &Pepper, pass: &str) -> Self {
            let hashing_algorithm = Algorithm::default();

            let ctx = Argon2::new_with_secret(
                pepper.as_bytes(),
                hashing_algorithm,
                Version::default(),
                Params::default(),
            ).unwrap();

            PasswordHash(
                ctx.hash_password(pass.as_bytes(), &salt).unwrap().serialize().to_string()
            )
        }
    }
}


pub mod user_data {
    use std::collections::HashSet;
    use crate::common_types::credentials::UserCredentials;
    use crate::common_types::Role;

    #[derive(Clone)]
    pub struct UserData {
        pub credentials: UserCredentials,
        pub roles: HashSet<Role>,
    }
}