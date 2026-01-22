use poem_openapi::Object;

/// User scheme.
#[derive(Object)]
#[oai(example = "user_example")]
pub struct User {
    /// User ID.
    pub id: i32,
    /// Username.
    pub username: String,
    #[oai(skip)]
    pub password_hash: String,
    /// Flag indicating the user is superuser or not.
    pub is_superuser: bool,
}

/// Create user scheme.
#[derive(Object)]
#[oai(example = "create_user_example")]
pub struct CreateUser {
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
}

pub struct InsertUser {
    pub username: String,
    pub password_hash: String,
    pub is_superuser: bool,
}

fn user_example() -> User {
    User {
        id: 1,
        username: "admin".to_string(),
        password_hash: "$2b$12$1dn.jSkFKobyQKMCbBxVc.7mcHZFz16dg/t3OFifRfE.6wJd2Vmei".to_string(),
        is_superuser: true,
    }
}

fn create_user_example() -> CreateUser {
    CreateUser {
        username: "user".to_string(),
        password: "12345".to_string(),
    }
}

impl From<crate::entity::users::Model> for User {
    fn from(model: crate::entity::users::Model) -> Self {
        User {
            id: model.id,
            username: model.username,
            password_hash: model.password_hash,
            is_superuser: model.is_super_user,
        }
    }
}
