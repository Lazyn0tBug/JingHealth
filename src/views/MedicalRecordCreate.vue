<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { api, type MedicalRecordCreateRequest } from '../services/api';

const router = useRouter();
const route = useRoute();
const tsid = route.query.tsid as string || '';

const loading = ref(false);
const error = ref('');

const form = ref<MedicalRecordCreateRequest>({
  chief_complaint: '',
  systolic_bp: undefined,
  diastolic_bp: undefined,
  temperature: undefined,
  height: undefined,
  weight: undefined,
  icd10_code: undefined,
  diagnosis: undefined,
  first_diagnosis_date: undefined,
  final_diagnosis: undefined,
  case_type: undefined,
  staging: undefined,
  second_opinion: undefined,
  referral_hospital: undefined,
  department: undefined,
  attending_doctor: undefined,
  treatment_plan: undefined,
  treatment_cycle: undefined,
  hospitalization: undefined,
  hospital_days: undefined,
  followup_status: undefined,
  current_status: undefined,
});

const caseTypes = ['重症', '体检', '抗衰', '其他'];
const followupStatuses = ['康复', '稳定', '治疗中'];

const computedBmi = computed(() => {
  if (form.value.height && form.value.weight && form.value.height > 0) {
    const hM = form.value.height / 100;
    return (form.value.weight / (hM * hM)).toFixed(1);
  }
  return null;
});

async function submit() {
  if (!form.value.chief_complaint) {
    error.value = '请填写主诉';
    return;
  }

  if (!tsid) {
    error.value = '缺少患者TSID';
    return;
  }

  loading.value = true;
  error.value = '';
  try {
    await api.createMedicalRecord(tsid, form.value);
    router.push(`/patients/${tsid}`);
  } catch (e: any) {
    error.value = '创建失败: ' + (e?.message || e);
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  if (!tsid) {
    error.value = '请从患者详情页进入新建就诊';
  }
});
</script>

<template>
  <div class="min-h-screen bg-base p-6">
    <div class="max-w-2xl mx-auto">
      <div class="flex items-center gap-4 mb-6">
        <button @click="router.back()" class="text-text-tertiary hover:text-text-secondary">← 返回</button>
        <h1 class="text-2xl font-semibold text-accent-secondary">新建就诊记录</h1>
        <span v-if="tsid" class="text-sm text-text-tertiary font-mono">患者: {{ tsid }}</span>
      </div>

      <form @submit.prevent="submit" class="medical-card p-6 space-y-6">
        <div v-if="error" class="alert-error">
          {{ error }}
        </div>

        <!-- 主诉 -->
        <div>
          <h3 class="text-sm font-semibold text-text-secondary mb-3 border-b border-subtle pb-2">主诉与体征</h3>
          <div>
            <label class="block text-sm font-medium text-text-secondary mb-1">主诉 <span class="text-red-500">*</span></label>
            <textarea
              v-model="form.chief_complaint"
              required
              rows="3"
              class="medical-input"
              placeholder="患者主要不适症状..."
            ></textarea>
          </div>

          <div class="grid grid-cols-2 gap-4 mt-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">收缩压 (mmHg)</label>
              <input v-model.number="form.systolic_bp" type="number" class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">舒张压 (mmHg)</label>
              <input v-model.number="form.diastolic_bp" type="number" class="medical-input" />
            </div>
          </div>

          <div class="grid grid-cols-3 gap-4 mt-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">体温 (°C)</label>
              <input v-model.number="form.temperature" type="number" step="0.1" class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">身高 (cm)</label>
              <input v-model.number="form.height" type="number" class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">体重 (kg)</label>
              <input v-model.number="form.weight" type="number" class="medical-input" />
            </div>
          </div>

          <div v-if="computedBmi" class="mt-2 text-sm text-text-secondary">BMI: {{ computedBmi }}</div>
        </div>

        <!-- 诊断 -->
        <div>
          <h3 class="text-sm font-semibold text-text-secondary mb-3 border-b border-subtle pb-2">诊断信息</h3>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">ICD-10 编码</label>
              <input v-model="form.icd10_code" class="medical-input" placeholder="如: J06.9" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">初诊日期</label>
              <input v-model="form.first_diagnosis_date" type="date" class="medical-input" />
            </div>
          </div>
          <div class="mt-4">
            <label class="block text-sm font-medium text-text-secondary mb-1">诊断</label>
            <input v-model="form.diagnosis" class="medical-input" />
          </div>
        </div>

        <!-- PMS 医疗核心信息 -->
        <div>
          <h3 class="text-sm font-semibold text-text-secondary mb-3 border-b border-subtle pb-2">医疗核心信息</h3>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">归类</label>
              <select v-model="form.case_type" class="medical-input">
                <option value="">请选择</option>
                <option v-for="ct in caseTypes" :key="ct" :value="ct">{{ ct }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">分期/严重程度</label>
              <input v-model="form.staging" class="medical-input" placeholder="如：III期" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">是否二诊意见</label>
              <select v-model="form.second_opinion" class="medical-input">
                <option value="">请选择</option>
                <option :value="true">是</option>
                <option :value="false">否</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">接诊医院</label>
              <input v-model="form.referral_hospital" class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">主治科室</label>
              <input v-model="form.department" class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">主治医生</label>
              <input v-model="form.attending_doctor" class="medical-input" />
            </div>
          </div>

          <div class="mt-4">
            <label class="block text-sm font-medium text-text-secondary mb-1">治疗方案</label>
            <input v-model="form.treatment_plan" class="medical-input" placeholder="手术/化疗/免疫/质子等" />
          </div>

          <div class="mt-4">
            <label class="block text-sm font-medium text-text-secondary mb-1">治疗周期</label>
            <input v-model="form.treatment_cycle" class="medical-input" />
          </div>

          <div class="grid grid-cols-2 gap-4 mt-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">是否住院</label>
              <select v-model="form.hospitalization" class="medical-input">
                <option value="">请选择</option>
                <option :value="true">是</option>
                <option :value="false">否</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">住院天数</label>
              <input v-model.number="form.hospital_days" type="number" class="medical-input" />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4 mt-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">复诊情况</label>
              <select v-model="form.followup_status" class="medical-input">
                <option value="">请选择</option>
                <option v-for="s in followupStatuses" :key="s" :value="s">{{ s }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">终诊</label>
              <input v-model="form.final_diagnosis" class="medical-input" />
            </div>
          </div>

          <div class="mt-4">
            <label class="block text-sm font-medium text-text-secondary mb-1">当前状况</label>
            <input v-model="form.current_status" class="medical-input" />
          </div>
        </div>

        <div class="flex gap-3 pt-2">
          <button
            type="submit"
            :disabled="loading || !tsid"
            class="btn-primary flex-1"
          >
            {{ loading ? '创建中...' : '创建就诊记录' }}
          </button>
          <button
            type="button"
            @click="router.back()"
            class="btn-secondary"
          >
            取消
          </button>
        </div>
      </form>
    </div>
  </div>
</template>