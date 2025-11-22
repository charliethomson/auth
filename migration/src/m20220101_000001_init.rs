use sea_orm_migration::{prelude::*, schema::*, sea_orm::prelude::DateTime};

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
                    .col(pk_auto(User::UserId))
                    .col(string(User::DisplayName).not_null())
                    .col(string(User::Username).not_null().unique_key())
                    .col(string(User::Password).not_null())
                    .col(boolean(User::Enabled).not_null().default(true))
                    .col(string_null(User::Email).default(None as Option<String>))
                    .col(string_null(User::ImageUrl).default(None as Option<String>))
                    .col(date_time_null(User::LastLogin).default(None as Option<DateTime>))
                    .col(string(User::CreatedBy).not_null())
                    .col(string(User::UpdatedBy).not_null())
                    .col(date_time(User::CreatedAt).not_null())
                    .col(date_time(User::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Application::Table)
                    .if_not_exists()
                    .col(string(Application::ApplicationId).primary_key().not_null())
                    .col(string(Application::DisplayName).not_null())
                    .col(string(Application::Description).not_null())
                    .col(string(Application::CreatedBy).not_null())
                    .col(string(Application::UpdatedBy).not_null())
                    .col(date_time(Application::CreatedAt).not_null())
                    .col(date_time(Application::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Grant::Table)
                    .if_not_exists()
                    .col(string(Grant::GrantId).primary_key().not_null())
                    .col(string(Grant::ApplicationId).not_null())
                    .col(string(Grant::DisplayName).not_null())
                    .col(string(Grant::Description).not_null())
                    .col(string(Grant::CreatedBy).not_null())
                    .col(string(Grant::UpdatedBy).not_null())
                    .col(date_time(Grant::CreatedAt).not_null())
                    .col(date_time(Grant::UpdatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Grant::Table, Grant::ApplicationId)
                            .to(Application::Table, Application::ApplicationId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserGrant::Table)
                    .if_not_exists()
                    .col(integer(UserGrant::UserId).not_null())
                    .col(string(UserGrant::GrantId).not_null())
                    .col(boolean(UserGrant::Enabled).not_null().default(true))
                    .col(date_time_null(UserGrant::EnabledAt))
                    .col(date_time_null(UserGrant::DisabledAt))
                    .col(string(UserGrant::CreatedBy).not_null())
                    .col(string(UserGrant::UpdatedBy).not_null())
                    .col(date_time(UserGrant::CreatedAt).not_null())
                    .col(date_time(UserGrant::UpdatedAt).not_null())
                    .primary_key(
                        Index::create()
                            .col(UserGrant::UserId)
                            .col(UserGrant::GrantId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserGrant::Table, UserGrant::GrantId)
                            .to(Grant::Table, Grant::GrantId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserGrant::Table, UserGrant::UserId)
                            .to(User::Table, User::UserId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGrant::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Grant::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Application::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    UserId,
    DisplayName,
    Username,
    Password,
    Enabled,
    Email,
    ImageUrl,
    LastLogin,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Grant {
    Table,
    GrantId,
    ApplicationId,
    DisplayName,
    Description,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Application {
    Table,
    ApplicationId,
    DisplayName,
    Description,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserGrant {
    Table,
    UserId,
    GrantId,
    Enabled,
    EnabledAt,
    DisabledAt,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
}
