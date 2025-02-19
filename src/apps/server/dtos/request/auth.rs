use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct SetUserPasswordRequestBody {
    pub user_id: String,
    pub password: String,
}
