use serde_json::Value;
use unicode_normalization::UnicodeNormalization;

use sea_orm::EntityTrait;
use crate::entities::patient::Entity as PatientEntity;

pub struct DedupCandidate {
    pub id: i64,
    pub tsid: String,
    pub name: String,
    pub birth_date: chrono::NaiveDate,
}

/// Returns patients where:
///   - name NFC-normalized Levenshtein distance <= 2, OR
///   - identical birth_date
pub async fn find_similar_patients(
    db: &sea_orm::DatabaseConnection,
    name: &str,
    birth_date: chrono::NaiveDate,
) -> Result<Vec<DedupCandidate>, sea_orm::DbErr> {
    let name_lower = name.to_lowercase().nfkd().collect::<String>();

    let all_patients: Vec<Value> = PatientEntity::find()
        .into_json()
        .all(db)
        .await?;

    let mut candidates = Vec::new();
    for patient_data in all_patients {
        let obj = match patient_data.as_object() {
            Some(o) => o,
            None => continue,
        };

        let p_name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let p_bdate_str = obj.get("birth_date").and_then(|v| v.as_str()).unwrap_or("");
        let p_bdate = chrono::NaiveDate::parse_from_str(p_bdate_str, "%Y-%m-%d").ok();

        let same_bdate = p_bdate.map(|d| d == birth_date).unwrap_or(false);
        let name_normalized = p_name.to_lowercase().nfkd().collect::<String>();
        let lev_dist = strsim::levenshtein(&name_normalized, &name_lower);
        let similar_name = lev_dist <= 2;

        if same_bdate || similar_name {
            let id = obj.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
            let tsid = obj.get("tsid").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if id > 0 {
                candidates.push(DedupCandidate {
                    id,
                    tsid,
                    name: p_name.to_string(),
                    birth_date: p_bdate.unwrap_or(birth_date),
                });
            }
        }
    }

    Ok(candidates)
}