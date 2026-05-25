<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { api, type PatientCreateRequest } from '../services/api';

const router = useRouter();
const loading = ref(false);
const error = ref('');

const form = ref<PatientCreateRequest>({
  name: '',
  gender: '',
  birth_date: '',
  phone: undefined,
  id_card: undefined,
  nationality: undefined,
  source_channel: undefined,
  first_time_to_japan: undefined,
  japanese_level: undefined,
  accompany_count: undefined,
  case_no: undefined,
  first_visit_date: undefined,
  allergy_tags: [],
  chronic_disease_tags: [],
  force_create: false,
});

const allergyInput = ref('');
const chronicInput = ref('');

const japaneseLevels = ['无', '基础', '流利'];
const sourceChannels = ['自媒体', '老客户', '代理', '其他'];

async function submit() {
  if (!form.value.name || !form.value.gender || !form.value.birth_date) {
    error.value = '请填写必填项';
    return;
  }

  if (allergyInput.value) {
    form.value.allergy_tags = allergyInput.value.split(',').map(s => s.trim()).filter(Boolean);
  }
  if (chronicInput.value) {
    form.value.chronic_disease_tags = chronicInput.value.split(',').map(s => s.trim()).filter(Boolean);
  }

  loading.value = true;
  error.value = '';
  try {
    await api.createPatient(form.value);
    router.push('/');
  } catch (e: any) {
    if (e?.message?.includes('409')) {
      error.value = '发现疑似重复患者，请检查或勾选"强制创建"。';
    } else {
      error.value = '创建失败: ' + (e?.message || e);
    }
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="min-h-screen bg-base p-6">
    <div class="max-w-2xl mx-auto">
      <div class="flex items-center gap-4 mb-6">
        <button @click="router.back()" class="text-text-tertiary hover:text-text-secondary">← 返回</button>
        <h1 class="text-2xl font-semibold text-accent-secondary">新建患者档案</h1>
      </div>

      <form @submit.prevent="submit" class="medical-card p-6 space-y-6">
        <div v-if="error" class="alert-error">
          {{ error }}
        </div>

        <!-- 基本信息 -->
        <div>
          <h3 class="text-sm font-semibold text-text-secondary mb-3 border-b border-subtle pb-2">基本信息</h3>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">姓名 <span class="text-red-500">*</span></label>
              <input v-model="form.name" required class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">性别 <span class="text-red-500">*</span></label>
              <select v-model="form.gender" required class="medical-input">
                <option value="">请选择</option>
                <option value="男">男</option>
                <option value="女">女</option>
                <option value="其他">其他</option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4 mt-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">出生日期 <span class="text-red-500">*</span></label>
              <input v-model="form.birth_date" type="date" required class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">案例编号</label>
              <input v-model="form.case_no" class="medical-input" placeholder="内部编号" />
            </div>
          </div>
        </div>

        <!-- 联系信息 -->
        <div>
          <h3 class="text-sm font-semibold text-text-secondary mb-3 border-b border-subtle pb-2">联系信息</h3>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">联系电话</label>
              <input v-model="form.phone" class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">身份证号</label>
              <input v-model="form.id_card" class="medical-input" />
            </div>
          </div>
        </div>

        <!-- PMS 扩展信息 -->
        <div>
          <h3 class="text-sm font-semibold text-text-secondary mb-3 border-b border-subtle pb-2">赴日医疗信息</h3>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">国籍</label>
              <input v-model="form.nationality" class="medical-input" placeholder="如：中国" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">来源渠道</label>
              <select v-model="form.source_channel" class="medical-input">
                <option value="">请选择</option>
                <option v-for="ch in sourceChannels" :key="ch" :value="ch">{{ ch }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">是否首次赴日</label>
              <select v-model="form.first_time_to_japan" class="medical-input">
                <option value="">请选择</option>
                <option :value="true">是</option>
                <option :value="false">否</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">日语能力</label>
              <select v-model="form.japanese_level" class="medical-input">
                <option value="">请选择</option>
                <option v-for="lv in japaneseLevels" :key="lv" :value="lv">{{ lv }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">陪同人数</label>
              <input v-model.number="form.accompany_count" type="number" min="0" class="medical-input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-text-secondary mb-1">接诊时间</label>
              <input v-model="form.first_visit_date" type="date" class="medical-input" />
            </div>
          </div>
        </div>

        <!-- 标签 -->
        <div>
          <h3 class="text-sm font-semibold text-text-secondary mb-3 border-b border-subtle pb-2">健康信息</h3>
          <div>
            <label class="block text-sm font-medium text-text-secondary mb-1">过敏标签（逗号分隔）</label>
            <input v-model="allergyInput" placeholder="如：青霉素,虾" class="medical-input" />
          </div>
          <div class="mt-4">
            <label class="block text-sm font-medium text-text-secondary mb-1">慢性病标签（逗号分隔）</label>
            <input v-model="chronicInput" placeholder="如：高血压,糖尿病" class="medical-input" />
          </div>
        </div>

        <div class="flex items-center gap-2">
          <input v-model="form.force_create" type="checkbox" id="force_create" class="rounded" />
          <label for="force_create" class="text-sm text-text-secondary">强制创建（跳过重复检查）</label>
        </div>

        <div class="flex gap-3 pt-2">
          <button
            type="submit"
            :disabled="loading"
            class="btn-primary flex-1"
          >
            {{ loading ? '创建中...' : '创建患者' }}
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