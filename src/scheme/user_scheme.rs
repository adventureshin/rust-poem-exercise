use poem_openapi::Object;

/// User scheme.
#[derive(Object)]
#[oai(example = "user_example")]
pub struct User {
    /// User ID.
    pub id: i32,
    /// Username.
    pub name: String,
    /// google id
    #[oai(skip)]
    pub google_id: String,
    /// email
    pub email: String,
    /// profile url
    pub profile_url: Option<String>,
    /// Flag indicating the user is superuser or not.
    pub is_superuser: bool,
}

/// Create user scheme.
#[derive(Object)]
#[oai(example = "create_user_example")]
pub struct CreateUser {
    pub name: String,
    pub google_id: String,
    pub email: String,
    pub profile_url: Option<String>,
    pub is_superuser: bool,
}

pub struct InsertUser {
    pub username: String,
    pub is_superuser: bool,
}

fn user_example() -> User {
    User {
        id: 1,
        name: "admin".to_string(),
        google_id: "google_id".to_string(),
        email: "test@gmail.com".to_string(),
        profile_url: Some("http://example.com/profile".to_string()),
        is_superuser: true,
    }
}

fn create_user_example() -> CreateUser {
    CreateUser {
        name: "user".to_string(),
        google_id: "google_id_2".to_string(),
        email: "testtest@gmail.com".to_string(),
        profile_url: Some("http://example.com/profile2".to_string()),
        is_superuser: false,
    }
}

impl From<crate::entity::users::Model> for User {
    fn from(model: crate::entity::users::Model) -> Self {
        User {
            id: model.id,
            name: model.name,
            google_id: model.google_id,
            email: model.email,
            profile_url: model.profile_url,
            is_superuser: model.is_super_user,
        }
    }
}
