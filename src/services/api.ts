// In development (npm run dev / Vite), requests are proxied through Vite's dev server.
// Vite proxies /api/* → http://127.0.0.1:1421 (see vite.config.ts).
// During production (dist/), the Vue app is served by the same Axum host.
const API_BASE = '/api/v1';

export interface Patient {
  id: number;
  tsid: string;
  name: string;
  gender: string;
  birth_date: string;
  phone: string | null;
  id_card: string | null;
  // PMS 扩展字段
  nationality: string | null;
  source_channel: string | null;
  first_time_to_japan: boolean | null;
  japanese_level: string | null;
  accompany_count: number | null;
  case_no: string | null;
  first_visit_date: string | null;
  allergy_tags: string[];
  chronic_disease_tags: string[];
  created_at: string;
  updated_at: string;
}

export interface PatientListResponse {
  patients: Patient[];
  total: number;
  page: number;
}

export interface PatientCreateRequest {
  name: string;
  gender: string;
  birth_date: string;
  phone?: string;
  id_card?: string;
  nationality?: string;
  source_channel?: string;
  first_time_to_japan?: boolean;
  japanese_level?: string;
  accompany_count?: number;
  case_no?: string;
  first_visit_date?: string;
  allergy_tags?: string[];
  chronic_disease_tags?: string[];
  force_create?: boolean;
}

export interface PatientUpdateRequest {
  name?: string;
  gender?: string;
  birth_date?: string;
  phone?: string;
  id_card?: string;
  nationality?: string;
  source_channel?: string;
  first_time_to_japan?: boolean;
  japanese_level?: string;
  accompany_count?: number;
  case_no?: string;
  first_visit_date?: string;
  allergy_tags?: string[];
  chronic_disease_tags?: string[];
}

export interface MedicalRecord {
  id: number;
  patient_id: number;
  chief_complaint: string;
  systolic_bp: number | null;
  diastolic_bp: number | null;
  temperature: number | null;
  height: number | null;
  weight: number | null;
  bmi: number | null;
  icd10_code: string | null;
  diagnosis: string | null;
  status: string;
  visit_date: string;
  // PMS 扩展字段
  first_diagnosis_date: string | null;
  final_diagnosis: string | null;
  case_type: string | null;
  staging: string | null;
  second_opinion: boolean | null;
  referral_hospital: string | null;
  department: string | null;
  attending_doctor: string | null;
  treatment_plan: string | null;
  treatment_cycle: string | null;
  hospitalization: boolean | null;
  hospital_days: number | null;
  followup_status: string | null;
  current_status: string | null;
  created_at: string;
  updated_at: string;
}

export interface MedicalRecordCreateRequest {
  chief_complaint: string;
  systolic_bp?: number;
  diastolic_bp?: number;
  temperature?: number;
  height?: number;
  weight?: number;
  icd10_code?: string;
  diagnosis?: string;
  // PMS 扩展字段
  first_diagnosis_date?: string;
  final_diagnosis?: string;
  case_type?: string;
  staging?: string;
  second_opinion?: boolean;
  referral_hospital?: string;
  department?: string;
  attending_doctor?: string;
  treatment_plan?: string;
  treatment_cycle?: string;
  hospitalization?: boolean;
  hospital_days?: number;
  followup_status?: string;
  current_status?: string;
}

async function fetchJson<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch(url, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });
  if (!res.ok) {
    throw new Error(`HTTP ${res.status}`);
  }
  return res.json();
}

export const api = {
  listPatients(page = 1, search?: string): Promise<PatientListResponse> {
    const params = new URLSearchParams({ page: String(page) });
    if (search) params.set('search', search);
    return fetchJson(`${API_BASE}/patients?${params}`);
  },

  getPatient(tsid: string): Promise<Patient> {
    return fetchJson(`${API_BASE}/patients/${tsid}`);
  },

  createPatient(data: PatientCreateRequest): Promise<Patient> {
    return fetchJson(`${API_BASE}/patients`, {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },

  updatePatient(tsid: string, data: PatientUpdateRequest): Promise<Patient> {
    return fetchJson(`${API_BASE}/patients/${tsid}`, {
      method: 'PATCH',
      body: JSON.stringify(data),
    });
  },

  listMedicalRecords(tsid: string): Promise<MedicalRecord[]> {
    return fetchJson(`${API_BASE}/patients/${tsid}/records`);
  },

  createMedicalRecord(tsid: string, data: MedicalRecordCreateRequest): Promise<MedicalRecord> {
    return fetchJson(`${API_BASE}/patients/${tsid}/records`, {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
};