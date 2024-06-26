use sea_orm::*;

use ::entity::{prelude::Users, users};

pub struct Query;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: i64) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id).one(db).await
    }
}
