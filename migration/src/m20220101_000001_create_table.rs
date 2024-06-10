use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .var_binary(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Users::Money)
                            .double()
                            .default(0.0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Gems)
                            .big_unsigned()
                            .default(0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    Money,
    Gems,
}
