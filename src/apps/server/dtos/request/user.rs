use fake::Fake as _;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::uroboros_user::uroboros_role::UroborosUserRole;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddUserRequestBody {
    pub login: String,
    pub role: UroborosUserRole,
    pub first_name: String,
    pub last_name: String,
    pub patronymick: Option<String>,
}

impl Default for AddUserRequestBody {
    fn default() -> Self {
        Self {
            role: UroborosUserRole::default(),
            login: fake::faker::internet::en::Username().fake::<String>(),
            first_name: fake::faker::name::en::FirstName().fake::<String>(),
            last_name: fake::faker::name::en::LastName().fake::<String>(),
            patronymick: None,
        }
    }
}
