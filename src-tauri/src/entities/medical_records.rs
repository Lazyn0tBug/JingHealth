use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "medical_records")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(index)]
    pub patient_id: i64,
    pub chief_complaint: String,
    pub systolic_bp: Option<i32>,
    pub diastolic_bp: Option<i32>,
    pub temperature: Option<f32>,
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub bmi: Option<f32>,
    pub icd10_code: Option<String>,
    pub diagnosis: Option<String>,
    #[sea_orm(default_value = "completed")]
    pub status: String,
    pub visit_date: DateTimeUtc,
    // PMS 扩展字段（医疗核心信息）
    pub first_diagnosis_date: Option<Date>,
    pub final_diagnosis: Option<String>,
    pub case_type: Option<String>,          // 重症/体检/抗衰
    pub staging: Option<String>,            // 分期/严重程度
    pub second_opinion: Option<bool>,       // 是否二诊意见
    pub referral_hospital: Option<String>, // 接诊医院
    pub department: Option<String>,         // 主治科室
    pub attending_doctor: Option<String>,  // 主治医生
    pub treatment_plan: Option<String>,     // 治疗方案
    pub treatment_cycle: Option<String>,    // 治疗周期
    pub hospitalization: Option<bool>,      // 是否住院
    pub hospital_days: Option<i32>,        // 住院天数
    pub followup_status: Option<String>,   // 复诊情况 康复/稳定/治疗中
    pub current_status: Option<String>,     // 当前状况
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}