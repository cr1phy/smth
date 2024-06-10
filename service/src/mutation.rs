use crate::inputs::UserInput;
use ::entity::{users, users::Entity as Users};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(db: &DbConn, data: UserInput) -> Result<users::ActiveModel, DbErr> {
        users::ActiveModel {
            name: Set(data.name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
