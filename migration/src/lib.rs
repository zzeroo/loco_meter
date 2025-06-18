#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250618_183302_meter_types;
mod m20250618_183832_units;
mod m20250618_184902_meters;
mod m20250618_185743_meter_entries;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250618_183302_meter_types::Migration),
            Box::new(m20250618_183832_units::Migration),
            Box::new(m20250618_184902_meters::Migration),
            Box::new(m20250618_185743_meter_entries::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}