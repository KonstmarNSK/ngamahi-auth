pub mod tokens {
    use serde::Deserialize;

    pub struct JWT {
        pub header: Header,
        pub payload: Payload,
        pub signature: Signature,
    }

    pub struct Header {}

    pub struct Payload {}

    pub struct Signature {}


    #[derive(Deserialize)]
    pub struct RefreshToken {
        pub username: String,
        pub password: String,
    }
}


pub mod credentials {
    use argon2::{Algorithm, Argon2, Params, Version};
    use password_hash::PasswordHasher;

    pub type Login = String;    // must be unique
    pub type Role = String;
    pub type Salt = String;
    pub type Pepper = String;

    #[derive(Clone)]
    pub struct UserCredentials {
        pub login: Login,
        pub pass: Password,
    }

    #[derive(Clone)]
    pub struct Password {
        pub salt: Salt,
        pub pass: PasswordHash,
    }

    #[derive(Clone)]
    pub struct PasswordHash(String);


    impl PasswordHash {
        pub fn new(salt: &Salt, pepper: &Pepper, pass: String) -> Self {
            let hashing_algorithm = Algorithm::default();

            let ctx = Argon2::new_with_secret(
                pepper.as_bytes(),
                hashing_algorithm,
                Version::default(),
                Params::default(),
            ).unwrap();

            PasswordHash(
                ctx.hash_password(&pass.as_bytes(), &salt).unwrap().serialize().to_string()
            )
        }
    }
}


pub mod user_data {
    use crate::common_types::credentials::{Role, UserCredentials};

    pub struct UserData {
        pub credentials: UserCredentials,
        pub roles: Vec<Role>,
    }
}