use sea_orm::{DbBackend, Schema};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{DeriveActiveEnum, EnumIter},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);

        manager
            .create_type(
                schema
                    .create_enum_from_active_enum::<AuthMethods>()
                    .expect("Auth method enum"),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("auth_methods")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(uuid("user_id"))
                    .col(string("identifier"))
                    .col(string("value"))
                    .col(boolean("verified"))
                    .col(ColumnDef::new("auth_type").custom("auth_method_type"))
                    .col(timestamp("created_at"))
                    .col(timestamp("updated_at"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_auth_methods_user_id_users_id")
                            .from("auth_methods", "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name("auth_method_type").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table("auth_methods").to_owned())
            .await?;

        manager
            .drop_foreign_key(ForeignKey::drop().name("FK_users_auth_method").to_owned())
            .await
    }
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "auth_method_type")]
pub enum AuthMethods {
    #[sea_orm(string_value = "Email")]
    Email,
    #[sea_orm(string_value = "Password")]
    Password,
}
