use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum Role {
    #[sea_orm(num_value = 1)]
    User,
    #[sea_orm(num_value = 2)]
    Subscriber,
    #[sea_orm(num_value = 5)]
    Admin,
}

impl From<u8> for Role {
    fn from(value: u8) -> Self {
        match value {
            1 => Role::User,
            2 => Role::Subscriber,
            5 => Role::Admin,
            _ => Role::User,
        }
    }
}

impl Into<u8> for Role {
    fn into(self) -> u8 {
        match self {
            Role::User => 1,
            Role::Subscriber => 2,
            Role::Admin => 5,
        }
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub avatar: String,
    pub password_hash: String,
    pub role: Role,
    pub settings: Json,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_one = "super::refresh_token::Entity",
        from = "Column::Id",
        to = "super::refresh_token::Column::UserId"
    )]
    RefreshToken,
}

impl Related<super::refresh_token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RefreshToken.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
