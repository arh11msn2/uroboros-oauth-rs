use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::uroboros_user::uroboros_role::UroborosUserRole;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "uroboros_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub role: UroborosUserRole,
    pub first_name: String,
    pub last_name: String,
    pub patronymick: Option<String>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::uroboros_user_auth_data_pg::Entity",
        from = "Column::Id",
        to = "super::uroboros_user_auth_data_pg::Column::UserId"
    )]
    AuthData,
}

impl Related<super::uroboros_user_auth_data_pg::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthData.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
