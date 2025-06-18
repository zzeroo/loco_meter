use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "meters",
            &[
            
            ("id", ColType::PkAuto),
            
            ("number", ColType::Integer),
            ("installation_date", ColType::DateTime),
            ("removal_date", ColType::DateTime),
            ],
            &[
            ("meter_type", ""),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "meters").await
    }
}
