use sea_orm::*;
use ::entity::{users, users::ActiveModel as User};

pub struct Mutation;

impl Mutation {
    pub async fn create_user() -> Result<users::ActiveModel, DbErr> {
        users::ActiveModel {
            
        }
    }
}