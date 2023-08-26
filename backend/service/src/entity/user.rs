use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub display_name: String,

    pub created: OffsetDateTime,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::session::Entity")]
    Sessions,

    #[sea_orm(has_one = "super::password_auth::Entity")]
    PasswordAuth,
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl Related<super::password_auth::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PasswordAuth.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
