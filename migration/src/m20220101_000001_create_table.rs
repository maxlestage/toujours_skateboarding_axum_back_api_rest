use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::USERNAME).string().not_null())
                    .col(ColumnDef::new(User::FIRSTNAME).string().not_null())
                    .col(ColumnDef::new(User::LASTNAME).string().not_null())
                    .col(ColumnDef::new(User::CELLNUMBER).string().not_null())
                    .col(ColumnDef::new(User::MAIL).string().not_null())
                    .col(ColumnDef::new(User::PASSWORD).string().not_null())
                    .col(ColumnDef::new(User::SECRETKEY).string().not_null())
                    .col(ColumnDef::new(User::REGISTRATEDDATE).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    USERNAME,
    FIRSTNAME,
    LASTNAME,
    CELLNUMBER,
    MAIL,
    PASSWORD,
    SECRETKEY,
    REGISTRATEDDATE,
}
