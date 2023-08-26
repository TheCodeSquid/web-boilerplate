use sea_orm_migration::prelude::*;
use ForeignKeyAction::*;

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    DisplayName,
    Created,
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    UserId,
    Start,
}

#[derive(DeriveIden)]
enum PasswordAuth {
    Table,
    UserId,
    Hash,
    QueueRehash,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::DisplayName).string().not_null())
                    .col(
                        ColumnDef::new(Users::Created)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sessions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Sessions::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Sessions::Start)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user")
                            .from(Sessions::Table, Sessions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_update(Cascade)
                            .on_delete(Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PasswordAuth::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PasswordAuth::UserId)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PasswordAuth::Hash).string().not_null())
                    .col(
                        ColumnDef::new(PasswordAuth::QueueRehash)
                            .boolean()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user")
                            .from(PasswordAuth::Table, PasswordAuth::UserId)
                            .to(Users::Table, Users::Id)
                            .on_update(Cascade)
                            .on_delete(Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PasswordAuth::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}
