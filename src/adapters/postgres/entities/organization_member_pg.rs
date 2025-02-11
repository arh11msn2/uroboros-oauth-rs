use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::organization::member_role::OrganizationMemberRole;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "organization_member")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role: OrganizationMemberRole,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization_pg::Entity",
        from = "Column::OrganizationId",
        to = "super::organization_pg::Column::Id"
    )]
    Organization,
}

impl Related<super::organization_pg::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
