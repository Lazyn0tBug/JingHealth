<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { api, type Patient, type MedicalRecord } from '../services/api';

const route = useRoute();
const router = useRouter();
const tsid = route.params.tsid as string;

const patient = ref<Patient | null>(null);
const records = ref<MedicalRecord[]>([]);
const loading = ref(true);
const activeTab = ref<'basic' | 'medical'>('basic');

// Navigation state: list of all patients TSIDs for prev/next
const allTsids = ref<string[]>([]);
const currentIdx = ref(-1);

async function load() {
  loading.value = true;
  try {
    patient.value = await api.getPatient(tsid);
    records.value = await api.listMedicalRecords(tsid);
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
}

async function loadAllTsids() {
  // Load all patient tsids for prev/next navigation
  try {
    const resp = await api.listPatients(1, '');
    allTsids.value = resp.patients.map((p: Patient) => p.tsid);
    currentIdx.value = allTsids.value.indexOf(tsid);
  } catch (e) {
    console.error(e);
  }
}

function formatDate(d: string) {
  return d ? d.split('T')[0] : '';
}

function formatDateTime(d: string) {
  return d ? d.replace('T', ' ').split('.')[0] : '';
}

// Latest record for medical tab
const latestRecord = computed(() => records.value[0] ?? null);

function prevPatient() {
  if (currentIdx.value > 0) {
    router.push(`/patients/${allTsids.value[currentIdx.value - 1]}/view`);
  }
}

function nextPatient() {
  if (currentIdx.value < allTsids.value.length - 1) {
    router.push(`/patients/${allTsids.value[currentIdx.value + 1]}/view`);
  }
}

onMounted(() => {
  load();
  loadAllTsids();
});
</script>

<template>
  <div class="min-h-screen bg-surface flex flex-col items-center justify-center p-6">
    <!-- Top nav -->
    <div class="w-full max-w-3xl flex items-center justify-between mb-4">
      <button @click="router.push('/')" class="text-text-tertiary hover:text-text-secondary text-sm">← 返回列表</button>
      <div class="flex gap-3">
        <button
          @click="prevPatient"
          :disabled="currentIdx <= 0"
          class="btn-secondary"
        >
          ← 上一个
        </button>
        <button
          @click="nextPatient"
          :disabled="currentIdx >= allTsids.length - 1"
          class="btn-secondary"
        >
          下一个 →
        </button>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="text-center py-20 text-text-secondary">加载中...</div>

    <template v-else-if="patient">
      <!-- Tab buttons -->
      <div class="w-full max-w-3xl flex gap-2 mb-4">
        <button
          @click="activeTab = 'basic'"
          :class="['px-6 py-2.5 rounded-lg text-sm font-medium transition-colors', activeTab === 'basic' ? 'bg-accent-primary text-inverse' : 'medical-card text-text-secondary hover:bg-surface']"
        >
          基本信息
        </button>
        <button
          @click="activeTab = 'medical'"
          :class="['px-6 py-2.5 rounded-lg text-sm font-medium transition-colors', activeTab === 'medical' ? 'bg-accent-primary text-inverse' : 'medical-card text-text-secondary hover:bg-surface']"
        >
          医疗核心信息
        </button>
      </div>

      <!-- Card -->
      <div class="w-full max-w-3xl medical-card rounded-2xl p-8">

        <!-- Header -->
        <div class="flex items-start justify-between mb-6 pb-6 border-b border-subtle">
          <div>
            <h1 class="text-3xl font-bold text-foreground">{{ patient.name }}</h1>
            <p class="text-sm text-text-tertiary mt-1 font-mono">TSID: {{ patient.tsid }}</p>
          </div>
          <span :class="['px-3 py-1 text-sm rounded-full font-medium', patient.gender === '男' ? 'gender-pill gender-pill--male' : 'gender-pill gender-pill--female']">
            {{ patient.gender }}
          </span>
        </div>

        <!-- Basic info tab -->
        <div v-if="activeTab === 'basic'" class="space-y-6">
          <div class="grid grid-cols-2 md:grid-cols-3 gap-6 text-sm">
            <div>
              <p class="text-text-tertiary text-xs mb-1">案例编号</p>
              <p class="font-semibold text-foreground font-mono">{{ patient.case_no ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-tertiary text-xs mb-1">出生日期</p>
              <p class="font-semibold text-foreground">{{ formatDate(patient.birth_date) }}</p>
            </div>
            <div>
              <p class="text-text-tertiary text-xs mb-1">国籍</p>
              <p class="font-semibold text-foreground">{{ patient.nationality ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-tertiary text-xs mb-1">来源渠道</p>
              <p class="font-semibold text-foreground">{{ patient.source_channel ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-tertiary text-xs mb-1">首次赴日</p>
              <p class="font-semibold text-foreground">{{ patient.first_time_to_japan ? '是' : patient.first_time_to_japan === false ? '否' : '—' }}</p>
            </div>
            <div>
              <p class="text-text-tertiary text-xs mb-1">日语能力</p>
              <p class="font-semibold text-foreground">{{ patient.japanese_level ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-tertiary text-xs mb-1">陪同人数</p>
              <p class="font-semibold text-foreground">{{ patient.accompany_count ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-tertiary text-xs mb-1">接诊时间</p>
              <p class="font-semibold text-foreground">{{ patient.first_visit_date ? formatDate(patient.first_visit_date) : '—' }}</p>
            </div>
            <div>
              <p class="text-text-tertiary text-xs mb-1">联系电话</p>
              <p class="font-semibold text-foreground">{{ patient.phone ?? '—' }}</p>
            </div>
          </div>

          <div v-if="patient.allergy_tags.length > 0 || patient.chronic_disease_tags.length > 0" class="space-y-2">
            <p class="text-text-tertiary text-xs">健康标签</p>
            <div class="flex flex-wrap gap-2">
              <span v-for="tag in patient.allergy_tags" :key="tag" class="status-pill status-pill--red">{{ tag }} (过敏)</span>
              <span v-for="tag in patient.chronic_disease_tags" :key="tag" class="status-pill status-pill--orange">{{ tag }} (慢性病)</span>
            </div>
          </div>
        </div>

        <!-- Medical info tab -->
        <div v-if="activeTab === 'medical'" class="space-y-6">
          <div v-if="!latestRecord" class="text-center py-10 text-text-tertiary">暂无就诊记录</div>
          <template v-else>
            <!-- Latest record header -->
            <div class="flex items-center justify-between mb-4 pb-4 border-b border-subtle">
              <div>
                <p class="text-xs text-text-tertiary">最近就诊</p>
                <p class="font-semibold text-text-secondary">{{ formatDateTime(latestRecord.visit_date) }}</p>
              </div>
              <span :class="['px-2 py-0.5 text-xs rounded-full', latestRecord.status === 'completed' ? 'status-pill status-pill--green' : 'bg-surface text-text-tertiary']">
                {{ latestRecord.status }}
              </span>
            </div>

            <div class="grid grid-cols-2 gap-y-6 gap-x-10 text-sm">
              <div>
                <p class="text-text-tertiary text-xs mb-1">主诉症状</p>
                <p class="font-medium text-foreground">{{ latestRecord.chief_complaint }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">归类</p>
                <p class="font-medium text-foreground">{{ latestRecord.case_type ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">初诊日期</p>
                <p class="font-medium text-foreground">{{ latestRecord.first_diagnosis_date ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">终诊</p>
                <p class="font-medium text-foreground">{{ latestRecord.final_diagnosis ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">分期/严重程度</p>
                <p class="font-medium text-foreground">{{ latestRecord.staging ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">二诊意见</p>
                <p class="font-medium text-foreground">{{ latestRecord.second_opinion !== null ? (latestRecord.second_opinion ? '是' : '否') : '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">接诊医院</p>
                <p class="font-medium text-foreground">{{ latestRecord.referral_hospital ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">主治科室</p>
                <p class="font-medium text-foreground">{{ latestRecord.department ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">主治医生</p>
                <p class="font-medium text-foreground">{{ latestRecord.attending_doctor ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">治疗方案</p>
                <p class="font-medium text-foreground">{{ latestRecord.treatment_plan ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">治疗周期</p>
                <p class="font-medium text-foreground">{{ latestRecord.treatment_cycle ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">住院</p>
                <p class="font-medium text-foreground">
                  <template v-if="latestRecord.hospitalization !== null">
                    {{ latestRecord.hospitalization ? '是' : '否' }}
                    <span v-if="latestRecord.hospital_days"> ({{ latestRecord.hospital_days }}天)</span>
                  </template>
                  <template v-else>—</template>
                </p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">复诊情况</p>
                <p class="font-medium text-foreground">{{ latestRecord.followup_status ?? '—' }}</p>
              </div>
              <div>
                <p class="text-text-tertiary text-xs mb-1">当前状况</p>
                <p class="font-medium text-foreground">{{ latestRecord.current_status ?? '—' }}</p>
              </div>
            </div>

            <!-- Vitals if available -->
            <div v-if="latestRecord.systolic_bp || latestRecord.temperature || latestRecord.height" class="pt-4 border-t border-subtle">
              <p class="text-text-tertiary text-xs mb-3">体征数据</p>
              <div class="flex flex-wrap gap-6 text-sm">
                <div v-if="latestRecord.systolic_bp">
                  <span class="text-text-secondary">血压</span>
                  <span class="ml-2 font-medium text-foreground">{{ latestRecord.systolic_bp }}/{{ latestRecord.diastolic_bp }} mmHg</span>
                </div>
                <div v-if="latestRecord.temperature">
                  <span class="text-text-secondary">体温</span>
                  <span class="ml-2 font-medium text-foreground">{{ latestRecord.temperature }} °C</span>
                </div>
                <div v-if="latestRecord.height && latestRecord.weight">
                  <span class="text-text-secondary">身高/体重</span>
                  <span class="ml-2 font-medium text-foreground">{{ latestRecord.height }}cm / {{ latestRecord.weight }}kg</span>
                </div>
                <div v-if="latestRecord.bmi">
                  <span class="text-text-secondary">BMI</span>
                  <span class="ml-2 font-medium text-foreground">{{ latestRecord.bmi.toFixed(1) }}</span>
                </div>
                <div v-if="latestRecord.icd10_code">
                  <span class="text-text-secondary">ICD-10</span>
                  <span class="ml-2 font-mono text-foreground">{{ latestRecord.icd10_code }}</span>
                </div>
              </div>
            </div>

            <!-- History records count -->
            <div v-if="records.length > 1" class="pt-4 border-t border-subtle text-sm text-text-tertiary">
              共 {{ records.length }} 条就诊记录 —
              <button @click="router.push(`/patients/${tsid}`)" class="text-accent-secondary hover:underline">查看全部</button>
            </div>
          </template>
        </div>
      </div>
    </template>
  </div>
</template>