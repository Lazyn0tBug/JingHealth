use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use sea_orm::{entity::EntityTrait, ActiveModelTrait, ColumnTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::axum_server::SharedState;
use crate::entities::medical_records;
use crate::entities::patient::Entity as PatientEntity;

#[derive(Debug, Deserialize)]
pub struct MedicalRecordCreateRequest {
    pub chief_complaint: String,
    pub systolic_bp: Option<i32>,
    pub diastolic_bp: Option<i32>,
    pub temperature: Option<f32>,
    pub height: Option<f32>,
    pub weight: Option<f32>,
    pub icd10_code: Option<String>,
    pub diagnosis: Option<String>,
    // PMS 扩展字段
    pub first_diagnosis_date: Option<String>,
    pub final_diagnosis: Option<String>,
    pub case_type: Option<String>,
    pub staging: Option<String>,
    pub second_opinion: Option<bool>,
    pub referral_hospital: Option<String>,
    pub department: Option<String>,
    pub attending_doctor: Option<String>,
    pub treatment_plan: Option<String>,
    pub treatment_cycle: Option<String>,
    pub hospitalization: Option<bool>,
    pub hospital_days: Option<i32>,
    pub followup_status: Option<String>,
    pub current_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MedicalRecordDto {
    pub id: i64,
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
    pub status: String,
    pub visit_date: String,
    // PMS 扩展字段
    pub first_diagnosis_date: Option<String>,
    pub final_diagnosis: Option<String>,
    pub case_type: Option<String>,
    pub staging: Option<String>,
    pub second_opinion: Option<bool>,
    pub referral_hospital: Option<String>,
    pub department: Option<String>,
    pub attending_doctor: Option<String>,
    pub treatment_plan: Option<String>,
    pub treatment_cycle: Option<String>,
    pub hospitalization: Option<bool>,
    pub hospital_days: Option<i32>,
    pub followup_status: Option<String>,
    pub current_status: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

fn to_dto(m: &medical_records::Model) -> MedicalRecordDto {
    MedicalRecordDto {
        id: m.id,
        patient_id: m.patient_id,
        chief_complaint: m.chief_complaint.clone(),
        systolic_bp: m.systolic_bp,
        diastolic_bp: m.diastolic_bp,
        temperature: m.temperature,
        height: m.height,
        weight: m.weight,
        bmi: m.bmi,
        icd10_code: m.icd10_code.clone(),
        diagnosis: m.diagnosis.clone(),
        status: m.status.clone(),
        visit_date: m.visit_date.to_rfc3339(),
        first_diagnosis_date: m.first_diagnosis_date.map(|d| d.format("%Y-%m-%d").to_string()),
        final_diagnosis: m.final_diagnosis.clone(),
        case_type: m.case_type.clone(),
        staging: m.staging.clone(),
        second_opinion: m.second_opinion,
        referral_hospital: m.referral_hospital.clone(),
        department: m.department.clone(),
        attending_doctor: m.attending_doctor.clone(),
        treatment_plan: m.treatment_plan.clone(),
        treatment_cycle: m.treatment_cycle.clone(),
        hospitalization: m.hospitalization,
        hospital_days: m.hospital_days,
        followup_status: m.followup_status.clone(),
        current_status: m.current_status.clone(),
        created_at: m.created_at.to_rfc3339(),
        updated_at: m.updated_at.to_rfc3339(),
    }
}

pub async fn create_medical_record(
    State(state): State<SharedState>,
    Path(tsid): Path<String>,
    Json(req): Json<MedicalRecordCreateRequest>,
) -> Result<(StatusCode, Json<MedicalRecordDto>), StatusCode> {
    use sea_orm::ActiveModelTrait;

    let patients: Vec<serde_json::Value> = PatientEntity::find()
        .into_json()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let patient = patients
        .into_iter()
        .find(|p| p.as_object().and_then(|o| o.get("tsid").and_then(|v| v.as_str())).map(|t| t == tsid).unwrap_or(false))
        .ok_or(StatusCode::NOT_FOUND)?;

    let patient_id = patient.as_object().and_then(|o| o.get("id").and_then(|v| v.as_i64())).unwrap_or(0);

    let bmi = match (req.height, req.weight) {
        (Some(h), Some(w)) if h > 0.0 && w > 0.0 => {
            let h_m = h / 100.0;
            Some((w as f32) / (h_m * h_m))
        }
        _ => None,
    };

    let first_diagnosis_date = if let Some(ref s) = req.first_diagnosis_date {
        chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
    } else {
        None
    };

    let now = chrono::Utc::now();
    let model = medical_records::ActiveModel {
        patient_id: sea_orm::Set(patient_id),
        chief_complaint: sea_orm::Set(req.chief_complaint),
        systolic_bp: sea_orm::Set(req.systolic_bp),
        diastolic_bp: sea_orm::Set(req.diastolic_bp),
        temperature: sea_orm::Set(req.temperature),
        height: sea_orm::Set(req.height),
        weight: sea_orm::Set(req.weight),
        bmi: sea_orm::Set(bmi),
        icd10_code: sea_orm::Set(req.icd10_code),
        diagnosis: sea_orm::Set(req.diagnosis),
        status: sea_orm::Set("completed".to_string()),
        visit_date: sea_orm::Set(now),
        first_diagnosis_date: sea_orm::Set(first_diagnosis_date),
        final_diagnosis: sea_orm::Set(req.final_diagnosis),
        case_type: sea_orm::Set(req.case_type),
        staging: sea_orm::Set(req.staging),
        second_opinion: sea_orm::Set(req.second_opinion),
        referral_hospital: sea_orm::Set(req.referral_hospital),
        department: sea_orm::Set(req.department),
        attending_doctor: sea_orm::Set(req.attending_doctor),
        treatment_plan: sea_orm::Set(req.treatment_plan),
        treatment_cycle: sea_orm::Set(req.treatment_cycle),
        hospitalization: sea_orm::Set(req.hospitalization),
        hospital_days: sea_orm::Set(req.hospital_days),
        followup_status: sea_orm::Set(req.followup_status),
        current_status: sea_orm::Set(req.current_status),
        created_at: sea_orm::Set(now),
        updated_at: sea_orm::Set(now),
        ..Default::default()
    };

    let record = model.insert(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(to_dto(&record))))
}

pub async fn list_medical_records(
    State(state): State<SharedState>,
    Path(tsid): Path<String>,
) -> Result<Json<Vec<MedicalRecordDto>>, StatusCode> {
    let patients: Vec<serde_json::Value> = PatientEntity::find()
        .into_json()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let patient = patients
        .into_iter()
        .find(|p| p.as_object().and_then(|o| o.get("tsid").and_then(|v| v.as_str())).map(|t| t == tsid).unwrap_or(false))
        .ok_or(StatusCode::NOT_FOUND)?;

    let patient_id = patient.as_object().and_then(|o| o.get("id").and_then(|v| v.as_i64())).unwrap_or(0);

    let records: Vec<medical_records::Model> = medical_records::Entity::find()
        .filter(medical_records::Column::PatientId.eq(patient_id))
        .order_by_desc(medical_records::Column::VisitDate)
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dtos: Vec<MedicalRecordDto> = records.iter().map(to_dto).collect();
    Ok(Json(dtos))
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/api/v1/patients/{tsid}/records", post(create_medical_record).get(list_medical_records))
        .route("/api/v1/medical_records/{id}", get(|| async { StatusCode::NOT_FOUND }))
        .with_state(state)
}