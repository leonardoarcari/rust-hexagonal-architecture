//! SeaORM Entity. Generated by sea-orm-codegen 0.2.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
