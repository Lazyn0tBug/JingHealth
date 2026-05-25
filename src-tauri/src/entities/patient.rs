use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "patients")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(unique, column_type = "Text")]
    pub tsid: String,
    pub name: String,
    pub gender: String,
    pub birth_date: Date,
    pub phone: Option<String>,
    pub id_card: Option<String>,
    // PMS 扩展字段
    pub nationality: Option<String>,
    pub source_channel: Option<String>,
    pub first_time_to_japan: Option<bool>,
    pub japanese_level: Option<String>,
    pub accompany_count: Option<i32>,
    pub case_no: Option<String>,
    pub first_visit_date: Option<DateTimeUtc>,
    // 原有标签字段保留
    pub allergy_tags: Option<String>,
    pub chronic_disease_tags: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}