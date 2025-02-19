use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetOrganizationPageRouteQuery {
    pub page: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct AddOrganizationRouteBody {
    pub name: String,
    pub description: Option<String>,
}
