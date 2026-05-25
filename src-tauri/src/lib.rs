use axum::Router;
use sea_orm::{Database, DbErr, DatabaseBackend, Statement, ConnectionTrait};
use std::sync::Arc;
use tokio::net::TcpListener;
use tauri::Manager;

mod axum_server;
mod entities;
mod handlers;
mod services;
mod snowflake;

use axum_server::{create_router as create_patient_router, AppState, SharedState};
use handlers::medical_record_handler::create_router as create_mr_router;
use snowflake::SnowflakeGenerator;

async fn init_db(app_data_dir: &std::path::Path) -> Result<sea_orm::DatabaseConnection, DbErr> {
    use sea_orm::Statement;

    std::fs::create_dir_all(app_data_dir).ok();
    let db_path = app_data_dir.join("pims.db");
    let db_url = format!(
        "sqlite:{}?mode=rwc",
        db_path.display()
    );
    let pool = sea_orm::Database::connect(&db_url).await?;

    // Auto-create patients table
    pool.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS patients (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            tsid TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            gender TEXT NOT NULL,
            birth_date DATE NOT NULL,
            phone TEXT,
            id_card TEXT,
            nationality TEXT,
            source_channel TEXT,
            first_time_to_japan INTEGER,
            japanese_level TEXT,
            accompany_count INTEGER,
            case_no TEXT,
            first_visit_date DATETIME,
            allergy_tags TEXT,
            chronic_disease_tags TEXT,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#,
    ))
    .await?;

    // ALTER TABLE patients — add new PMS columns (IF NOT EXISTS skips if already present)
    let patient_alters = [
        "ALTER TABLE patients ADD COLUMN IF NOT EXISTS nationality TEXT",
        "ALTER TABLE patients ADD COLUMN IF NOT EXISTS source_channel TEXT",
        "ALTER TABLE patients ADD COLUMN IF NOT EXISTS first_time_to_japan INTEGER",
        "ALTER TABLE patients ADD COLUMN IF NOT EXISTS japanese_level TEXT",
        "ALTER TABLE patients ADD COLUMN IF NOT EXISTS accompany_count INTEGER",
        "ALTER TABLE patients ADD COLUMN IF NOT EXISTS case_no TEXT",
        "ALTER TABLE patients ADD COLUMN IF NOT EXISTS first_visit_date DATETIME",
    ];
    for sql in patient_alters {
        pool.execute(Statement::from_string(DatabaseBackend::Sqlite, sql))
            .await
            .ok();
    }

    // Auto-create medical_records table
    pool.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        r#"
        CREATE TABLE IF NOT EXISTS medical_records (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            patient_id INTEGER NOT NULL,
            chief_complaint TEXT NOT NULL,
            systolic_bp INTEGER,
            diastolic_bp INTEGER,
            temperature REAL,
            height REAL,
            weight REAL,
            bmi REAL,
            icd10_code TEXT,
            diagnosis TEXT,
            status TEXT NOT NULL DEFAULT 'completed',
            visit_date DATETIME NOT NULL,
            first_diagnosis_date DATE,
            final_diagnosis TEXT,
            case_type TEXT,
            staging TEXT,
            second_opinion INTEGER,
            referral_hospital TEXT,
            department TEXT,
            attending_doctor TEXT,
            treatment_plan TEXT,
            treatment_cycle TEXT,
            hospitalization INTEGER,
            hospital_days INTEGER,
            followup_status TEXT,
            current_status TEXT,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL,
            FOREIGN KEY (patient_id) REFERENCES patients(id)
        )
        "#,
    ))
    .await?;

    // ALTER TABLE medical_records — add new PMS columns
    let mr_alters = [
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS first_diagnosis_date DATE",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS final_diagnosis TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS case_type TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS staging TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS second_opinion INTEGER",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS referral_hospital TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS department TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS attending_doctor TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS treatment_plan TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS treatment_cycle TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS hospitalization INTEGER",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS hospital_days INTEGER",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS followup_status TEXT",
        "ALTER TABLE medical_records ADD COLUMN IF NOT EXISTS current_status TEXT",
    ];
    for sql in mr_alters {
        pool.execute(Statement::from_string(DatabaseBackend::Sqlite, sql))
            .await
            .ok();
    }

    Ok(pool)
}

async fn start_axum_server(state: SharedState) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let patient_router = create_patient_router(state.clone());
    let mr_router = create_mr_router(state.clone());

    let app = Router::new()
        .merge(patient_router)
        .merge(mr_router);

    let addr = "127.0.0.1:1421";
    let listener = TcpListener::bind(addr).await?;
    println!("[Mini-PIMS] Axum server listening on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");

            let rt = tokio::runtime::Runtime::new()?;
            let pool = rt.block_on(async { init_db(&app_data_dir).await })?;

            let snowflake = Arc::new(SnowflakeGenerator::new(1));
            let state: SharedState = Arc::new(AppState {
                db: pool,
                snowflake: snowflake.clone(),
            });

            rt.spawn(async move {
                if let Err(e) = start_axum_server(state).await {
                    eprintln!("[Mini-PIMS] Axum server error: {}", e);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}