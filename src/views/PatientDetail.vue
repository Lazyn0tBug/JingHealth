<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { api, type Patient, type MedicalRecord } from '../services/api';
import Icon from '../components/Icon.vue';

const route = useRoute();
const router = useRouter();
const tsid = route.params.tsid as string;

const patient = ref<Patient | null>(null);
const records = ref<MedicalRecord[]>([]);
const loading = ref(true);

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

function formatDate(d: string) {
  return d ? d.split('T')[0] : '';
}

function formatDateTime(d: string) {
  return d ? d.replace('T', ' ').split('.')[0] : '';
}

onMounted(load);
</script>

<template>
  <AppLayout title="患者详情" back-href="/" padding="p-4 sm:p-6">
    <template #headerExtra>
      <button
        @click="router.push(`/records/new?tsid=${tsid}`)"
        class="btn-primary flex items-center gap-1"
      >
        <Icon name="add-circle-line" size="base" />新建就诊
      </button>
    </template>

      <div v-if="loading" class="text-center py-8 text-text-secondary">加载中...</div>

      <template v-else-if="patient">
        <!-- 基本信息卡片 -->
        <div class="medical-card p-4 sm:p-6">
          <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-3 mb-4">
            <div>
              <h2 class="text-xl font-semibold text-foreground">{{ patient.name }}</h2>
              <p class="text-sm text-text-tertiary mt-1">TSID: <span class="font-mono">{{ patient.tsid }}</span></p>
            </div>
          </div>

          <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
            <div>
              <p class="text-text-secondary">案例编号</p>
              <p class="font-medium text-foreground">{{ patient.case_no ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-secondary">性别</p>
              <p class="font-medium text-foreground">{{ patient.gender }}</p>
            </div>
            <div>
              <p class="text-text-secondary">出生日期</p>
              <p class="font-medium text-foreground">{{ formatDate(patient.birth_date) }}</p>
            </div>
            <div>
              <p class="text-text-secondary">国籍</p>
              <p class="font-medium text-foreground">{{ patient.nationality ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-secondary">联系电话</p>
              <p class="font-medium text-foreground">{{ patient.phone ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-secondary">来源渠道</p>
              <p class="font-medium text-foreground">{{ patient.source_channel ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-secondary">首次赴日</p>
              <p class="font-medium text-foreground">{{ patient.first_time_to_japan ? '是' : patient.first_time_to_japan === false ? '否' : '—' }}</p>
            </div>
            <div>
              <p class="text-text-secondary">日语能力</p>
              <p class="font-medium text-foreground">{{ patient.japanese_level ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-secondary">陪同人数</p>
              <p class="font-medium text-foreground">{{ patient.accompany_count ?? '—' }}</p>
            </div>
            <div>
              <p class="text-text-secondary">接诊时间</p>
              <p class="font-medium text-foreground">{{ patient.first_visit_date ? formatDate(patient.first_visit_date) : '—' }}</p>
            </div>
          </div>

          <div v-if="patient.allergy_tags.length > 0" class="mt-4">
            <p class="text-sm text-text-secondary">过敏标签</p>
            <div class="flex flex-wrap gap-1 mt-1">
              <span v-for="tag in patient.allergy_tags" :key="tag" class="status-pill status-pill--red">{{ tag }}</span>
            </div>
          </div>

          <div v-if="patient.chronic_disease_tags.length > 0" class="mt-4">
            <p class="text-sm text-text-secondary">慢性病</p>
            <div class="flex flex-wrap gap-1 mt-1">
              <span v-for="tag in patient.chronic_disease_tags" :key="tag" class="status-pill status-pill--orange">{{ tag }}</span>
            </div>
          </div>
        </div>

        <!-- 就诊记录列表 -->
        <div class="medical-card overflow-hidden">
          <div class="px-6 py-4 border-b border-subtle">
            <h3 class="font-semibold text-foreground">就诊记录 ({{ records.length }})</h3>
          </div>
          <div class="divide-y divide-subtle">
            <div v-for="r in records" :key="r.id" class="p-6 text-sm space-y-2">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <span class="font-medium text-foreground">{{ formatDateTime(r.visit_date) }}</span>
                  <span :class="['px-2 py-0.5 text-xs rounded-full', r.status === 'completed' ? 'status-pill status-pill--green' : 'bg-surface text-text-tertiary']">
                    {{ r.status }}
                  </span>
                </div>
                <span class="text-xs text-text-tertiary">ID: {{ r.id }}</span>
              </div>

              <p class="text-text-secondary">主诉: {{ r.chief_complaint }}</p>

              <!-- 体征 -->
              <div class="grid grid-cols-4 gap-2 text-xs text-text-tertiary">
                <span v-if="r.systolic_bp">血压: {{ r.systolic_bp }}/{{ r.diastolic_bp }} mmHg</span>
                <span v-if="r.temperature">体温: {{ r.temperature }} °C</span>
                <span v-if="r.height && r.weight">身高: {{ r.height }}cm 体重: {{ r.weight }}kg</span>
                <span v-if="r.bmi">BMI: {{ r.bmi.toFixed(1) }}</span>
              </div>

              <!-- PMS 医疗核心信息 -->
              <div v-if="r.case_type || r.final_diagnosis || r.staging" class="mt-2 p-3 bg-surface rounded-lg text-xs space-y-1">
                <p v-if="r.case_type" class="text-text-secondary">归类: {{ r.case_type }}</p>
                <p v-if="r.final_diagnosis" class="text-text-secondary">终诊: {{ r.final_diagnosis }}</p>
                <p v-if="r.staging" class="text-text-secondary">分期: {{ r.staging }}</p>
                <p v-if="r.second_opinion !== null" class="text-text-secondary">二诊意见: {{ r.second_opinion ? '是' : '否' }}</p>
                <p v-if="r.referral_hospital" class="text-text-secondary">接诊医院: {{ r.referral_hospital }}</p>
                <p v-if="r.department" class="text-text-secondary">主治科室: {{ r.department }}</p>
                <p v-if="r.attending_doctor" class="text-text-secondary">主治医生: {{ r.attending_doctor }}</p>
                <p v-if="r.treatment_plan" class="text-text-secondary">治疗方案: {{ r.treatment_plan }}</p>
                <p v-if="r.treatment_cycle" class="text-text-secondary">治疗周期: {{ r.treatment_cycle }}</p>
                <p v-if="r.hospitalization !== null" class="text-text-secondary">住院: {{ r.hospitalization ? '是' : '否' }}{{ r.hospital_days ? ` (${r.hospital_days}天)` : '' }}</p>
                <p v-if="r.followup_status" class="text-text-secondary">复诊情况: {{ r.followup_status }}</p>
                <p v-if="r.current_status" class="text-text-secondary">当前状况: {{ r.current_status }}</p>
              </div>
            </div>
            <div v-if="records.length === 0" class="p-6 text-center text-text-tertiary">暂无就诊记录</div>
          </div>
        </div>
      </template>
  </AppLayout>
</template>