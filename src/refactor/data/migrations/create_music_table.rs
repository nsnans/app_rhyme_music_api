use async_trait::async_trait;
use sea_orm_migration::{prelude::*, schema::pk_auto};

use crate::refactor::data::models::music_aggregator::Column;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MusicTable::Music)
                    .col(pk_auto(Column::Id))
                    .if_not_exists()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MusicTable::Music).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MusicTable {
    Music,
}
