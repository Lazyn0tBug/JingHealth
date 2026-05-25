use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use sea_orm::entity::EntityTrait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use unicode_normalization::UnicodeNormalization;

use crate::entities::patient;
use crate::snowflake::SnowflakeGenerator;

pub type DbPool = sea_orm::DatabaseConnection;
pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub db: DbPool,
    pub snowflake: Arc<SnowflakeGenerator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientDto {
    pub id: i64,
    pub tsid: String,
    pub name: String,
    pub gender: String,
    pub birth_date: String,
    pub phone: Option<String>,
    pub id_card: Option<String>,
    pub nationality: Option<String>,
    pub source_channel: Option<String>,
    pub first_time_to_japan: Option<bool>,
    pub japanese_level: Option<String>,
    pub accompany_count: Option<i32>,
    pub case_no: Option<String>,
    pub first_visit_date: Option<String>,
    pub allergy_tags: Vec<String>,
    pub chronic_disease_tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

fn mask_phone(phone: &str) -> String {
    if phone.len() < 7 {
        return phone.to_string();
    }
    format!("{}****{}****", &phone[..3], &phone[phone.len() - 4..])
}

fn parse_tags(s: &Option<String>) -> Vec<String> {
    s.as_ref()
        .and_then(|v| serde_json::from_str::<Vec<String>>(v).ok())
        .unwrap_or_default()
}

#[derive(Debug, Deserialize)]
pub struct PatientCreateRequest {
    pub name: String,
    pub gender: String,
    pub birth_date: String,
    pub phone: Option<String>,
    pub id_card: Option<String>,
    pub nationality: Option<String>,
    pub source_channel: Option<String>,
    pub first_time_to_japan: Option<bool>,
    pub japanese_level: Option<String>,
    pub accompany_count: Option<i32>,
    pub case_no: Option<String>,
    pub first_visit_date: Option<String>,
    pub allergy_tags: Option<Vec<String>>,
    pub chronic_disease_tags: Option<Vec<String>>,
    #[serde(default)]
    pub force_create: bool,
}

#[derive(Debug, Deserialize)]
pub struct PatientUpdateRequest {
    pub name: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub phone: Option<String>,
    pub id_card: Option<String>,
    pub nationality: Option<String>,
    pub source_channel: Option<String>,
    pub first_time_to_japan: Option<bool>,
    pub japanese_level: Option<String>,
    pub accompany_count: Option<i32>,
    pub case_no: Option<String>,
    pub first_visit_date: Option<String>,
    pub allergy_tags: Option<Vec<String>>,
    pub chronic_disease_tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PatientListQuery {
    pub search: Option<String>,
    #[serde(default = "default_page")]
    pub page: u32,
}

fn default_page() -> u32 {
    1
}

#[derive(Debug, Serialize)]
pub struct PaginatedPatientList {
    pub patients: Vec<PatientDto>,
    pub total: i64,
    pub page: u32,
}

fn json_to_patient_dto(p: &serde_json::Value) -> Option<PatientDto> {
    let obj = p.as_object()?;
    Some(PatientDto {
        id: obj.get("id")?.as_i64()?,
        tsid: obj.get("tsid")?.as_str()?.to_string(),
        name: obj.get("name")?.as_str()?.to_string(),
        gender: obj.get("gender")?.as_str()?.to_string(),
        birth_date: obj.get("birth_date")?.as_str()?.to_string(),
        phone: obj.get("phone").and_then(|v| v.as_str()).map(|s| mask_phone(s)),
        id_card: obj.get("id_card").and_then(|v| v.as_str()).map(|s| s.to_string()),
        nationality: obj.get("nationality").and_then(|v| v.as_str()).map(|s| s.to_string()),
        source_channel: obj.get("source_channel").and_then(|v| v.as_str()).map(|s| s.to_string()),
        first_time_to_japan: obj.get("first_time_to_japan").and_then(|v| v.as_i64()).map(|v| v != 0),
        japanese_level: obj.get("japanese_level").and_then(|v| v.as_str()).map(|s| s.to_string()),
        accompany_count: obj.get("accompany_count").and_then(|v| v.as_i64()).map(|v| v as i32),
        case_no: obj.get("case_no").and_then(|v| v.as_str()).map(|s| s.to_string()),
        first_visit_date: obj.get("first_visit_date").and_then(|v| v.as_str()).map(|s| s.to_string()),
        allergy_tags: obj.get("allergy_tags").and_then(|v| v.as_str()).and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default(),
        chronic_disease_tags: obj.get("chronic_disease_tags").and_then(|v| v.as_str()).and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default(),
        created_at: obj.get("created_at")?.as_str()?.to_string(),
        updated_at: obj.get("updated_at")?.as_str()?.to_string(),
    })
}

pub async fn list_patients(
    State(state): State<SharedState>,
    Query(query): Query<PatientListQuery>,
) -> Result<Json<PaginatedPatientList>, StatusCode> {
    let page_size = 20u32;
    let page = query.page.max(1);

    let all_patients: Vec<serde_json::Value> = patient::Entity::find()
        .into_json()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let filtered: Vec<serde_json::Value> = if let Some(q) = &query.search {
        let q_lower = q.to_lowercase().nfkd().collect::<String>();
        all_patients
            .into_iter()
            .filter(|p| {
                let name = p.as_object().and_then(|o| o.get("name").and_then(|v| v.as_str())).unwrap_or("");
                let tsid = p.as_object().and_then(|o| o.get("tsid").and_then(|v| v.as_str())).unwrap_or("");
                let case_no = p.as_object().and_then(|o| o.get("case_no").and_then(|v| v.as_str())).unwrap_or("");
                name.to_lowercase().contains(&q_lower) || tsid.to_lowercase().contains(&q_lower) || case_no.to_lowercase().contains(&q_lower)
            })
            .collect()
    } else {
        all_patients
    };

    let total = filtered.len() as i64;
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(filtered.len());
    let page_patients: Vec<PatientDto> = filtered[start..end]
        .iter()
        .filter_map(json_to_patient_dto)
        .collect();

    Ok(Json(PaginatedPatientList {
        patients: page_patients,
        total,
        page,
    }))
}

pub async fn get_patient(
    State(state): State<SharedState>,
    Path(tsid): Path<String>,
) -> Result<Json<PatientDto>, StatusCode> {
    let patients: Vec<serde_json::Value> = patient::Entity::find()
        .into_json()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let patient_json = patients
        .into_iter()
        .find(|p| p.as_object().and_then(|o| o.get("tsid").and_then(|v| v.as_str())).map(|t| t == tsid).unwrap_or(false))
        .ok_or(StatusCode::NOT_FOUND)?;

    json_to_patient_dto(&patient_json).ok_or(StatusCode::INTERNAL_SERVER_ERROR).map(Json)
}

pub async fn create_patient(
    State(state): State<SharedState>,
    Json(req): Json<PatientCreateRequest>,
) -> Result<(StatusCode, Json<PatientDto>), StatusCode> {
    use sea_orm::ActiveModelTrait;

    let birth_date = chrono::NaiveDate::parse_from_str(&req.birth_date, "%Y-%m-%d")
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    if !req.force_create {
        let candidates = crate::services::patient_dedup::find_similar_patients(&state.db, &req.name, birth_date)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if !candidates.is_empty() {
            return Err(StatusCode::CONFLICT);
        }
    }

    let tsid_str = state.snowflake.generate_str();
    let now = chrono::Utc::now();

    let first_visit_date = req.first_visit_date.as_ref().map(|_| chrono::Utc::now());

    let model = patient::ActiveModel {
        tsid: sea_orm::Set(tsid_str),
        name: sea_orm::Set(req.name),
        gender: sea_orm::Set(req.gender),
        birth_date: sea_orm::Set(birth_date),
        phone: sea_orm::Set(req.phone),
        id_card: sea_orm::Set(req.id_card),
        nationality: sea_orm::Set(req.nationality),
        source_channel: sea_orm::Set(req.source_channel),
        first_time_to_japan: sea_orm::Set(req.first_time_to_japan),
        japanese_level: sea_orm::Set(req.japanese_level),
        accompany_count: sea_orm::Set(req.accompany_count),
        case_no: sea_orm::Set(req.case_no),
        first_visit_date: sea_orm::Set(first_visit_date),
        allergy_tags: sea_orm::Set(req.allergy_tags.and_then(|v| serde_json::to_string(&v).ok())),
        chronic_disease_tags: sea_orm::Set(req.chronic_disease_tags.and_then(|v| serde_json::to_string(&v).ok())),
        created_at: sea_orm::Set(now),
        updated_at: sea_orm::Set(now),
        ..Default::default()
    };

    let patient = model.insert(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dto = PatientDto {
        id: patient.id,
        tsid: patient.tsid,
        name: patient.name,
        gender: patient.gender,
        birth_date: patient.birth_date.format("%Y-%m-%d").to_string(),
        phone: patient.phone.as_ref().map(|p| mask_phone(p)),
        id_card: patient.id_card,
        nationality: patient.nationality,
        source_channel: patient.source_channel,
        first_time_to_japan: patient.first_time_to_japan,
        japanese_level: patient.japanese_level,
        accompany_count: patient.accompany_count,
        case_no: patient.case_no,
        first_visit_date: patient.first_visit_date.map(|_| "".to_string()),
        allergy_tags: parse_tags(&patient.allergy_tags),
        chronic_disease_tags: parse_tags(&patient.chronic_disease_tags),
        created_at: patient.created_at.to_rfc3339(),
        updated_at: patient.updated_at.to_rfc3339(),
    };

    Ok((StatusCode::CREATED, Json(dto)))
}

pub async fn update_patient(
    State(state): State<SharedState>,
    Path(tsid): Path<String>,
    Json(req): Json<PatientUpdateRequest>,
) -> Result<Json<PatientDto>, StatusCode> {
    use sea_orm::ActiveModelTrait;

    let patients: Vec<serde_json::Value> = patient::Entity::find()
        .into_json()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let patient_json = patients
        .into_iter()
        .find(|p| p.as_object().and_then(|o| o.get("tsid").and_then(|v| v.as_str())).map(|t| t == tsid).unwrap_or(false))
        .ok_or(StatusCode::NOT_FOUND)?;

    let id = patient_json.as_object().and_then(|o| o.get("id").and_then(|v| v.as_i64())).unwrap_or(0);
    let mut model: patient::ActiveModel = patient::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into();

    if let Some(name) = req.name {
        model.name = sea_orm::Set(name);
    }
    if let Some(gender) = req.gender {
        model.gender = sea_orm::Set(gender);
    }
    if let Some(phone) = req.phone {
        model.phone = sea_orm::Set(Some(phone));
    }
    if let Some(id_card) = req.id_card {
        model.id_card = sea_orm::Set(Some(id_card));
    }
    if let Some(nationality) = req.nationality {
        model.nationality = sea_orm::Set(Some(nationality));
    }
    if let Some(source_channel) = req.source_channel {
        model.source_channel = sea_orm::Set(Some(source_channel));
    }
    if let Some(first_time_to_japan) = req.first_time_to_japan {
        model.first_time_to_japan = sea_orm::Set(Some(first_time_to_japan));
    }
    if let Some(japanese_level) = req.japanese_level {
        model.japanese_level = sea_orm::Set(Some(japanese_level));
    }
    if let Some(accompany_count) = req.accompany_count {
        model.accompany_count = sea_orm::Set(Some(accompany_count));
    }
    if let Some(case_no) = req.case_no {
        model.case_no = sea_orm::Set(Some(case_no));
    }
    if let Some(tags) = req.allergy_tags {
        model.allergy_tags = sea_orm::Set(serde_json::to_string(&tags).ok());
    }
    if let Some(tags) = req.chronic_disease_tags {
        model.chronic_disease_tags = sea_orm::Set(serde_json::to_string(&tags).ok());
    }
    model.updated_at = sea_orm::Set(chrono::Utc::now());

    let patient = model.update(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dto = PatientDto {
        id: patient.id,
        tsid: patient.tsid,
        name: patient.name,
        gender: patient.gender,
        birth_date: patient.birth_date.format("%Y-%m-%d").to_string(),
        phone: patient.phone.as_ref().map(|p| mask_phone(p)),
        id_card: patient.id_card,
        nationality: patient.nationality,
        source_channel: patient.source_channel,
        first_time_to_japan: patient.first_time_to_japan,
        japanese_level: patient.japanese_level,
        accompany_count: patient.accompany_count,
        case_no: patient.case_no,
        first_visit_date: patient.first_visit_date.map(|_| "".to_string()),
        allergy_tags: parse_tags(&patient.allergy_tags),
        chronic_disease_tags: parse_tags(&patient.chronic_disease_tags),
        created_at: patient.created_at.to_rfc3339(),
        updated_at: patient.updated_at.to_rfc3339(),
    };

    Ok(Json(dto))
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/api/v1/patients", post(create_patient).get(list_patients))
        .route("/api/v1/patients/{tsid}", get(get_patient).patch(update_patient))
        .with_state(state)
}