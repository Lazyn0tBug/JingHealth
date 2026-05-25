<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { api, type Patient, type PatientListResponse } from '../services/api';
import Icon from '../components/Icon.vue';

const router = useRouter();
const patients = ref<Patient[]>([]);
const total = ref(0);
const page = ref(1);
const search = ref('');
const loading = ref(false);
const viewMode = ref<'table' | 'cards'>('cards');

async function loadPatients() {
  loading.value = true;
  try {
    const resp: PatientListResponse = await api.listPatients(page.value, search.value || undefined);
    patients.value = resp.patients;
    total.value = resp.total;
  } catch (e) {
    console.error('Failed to load patients', e);
  } finally {
    loading.value = false;
  }
}

function formatDate(d: string) {
  return d ? d.split('T')[0] : '';
}

const mockLoading = ref(false);

async function createMockPatient() {
  mockLoading.value = true;
  try {
    const names = ['张三', '李四', '王五', '赵六', '钱七', '孙八', '周九', '吴十'];
    const genders = ['男', '女'];
    const nationalities = ['中国', '日本', '美国', '韩国'];
    const channels = ['自媒体', '老客户', '代理', '其他'];
    const japaneseLevels = ['无', '基础', '流利'];
    const caseTypes = ['重症', '体检', '抗衰', '其他'];
    const followupStatuses = ['康复', '稳定', '治疗中'];
    const departments = ['内科', '外科', '肿瘤科', '体检中心', '妇科'];
    const hospitals = ['东京大学附属病院', '癌研有明病院', '顺天堂医院', '国立癌症研究中心'];
    const doctors = ['佐藤医生', '田中医生', '山本医生', '伊藤医生'];
    const treatments = ['手术切除', '化疗', '免疫治疗', '质子治疗', '观察随访'];
    const complaints = [
      '体检发现肺部阴影，经进一步检查确诊',
      '反复腹痛伴有血便，内镜检查发现异常',
      '甲状腺结节穿刺结果提示恶性可能',
      '乳腺癌术后复查，影像学评估',
      '胃癌根治术后常规复诊',
      'PET-CT检查发现淋巴结转移',
      '子宫内膜增厚，刮宫病理回报',
      '体检CA199升高，腹部CT检查',
    ];

    const patient = await api.createPatient({
      name: names[Math.floor(Math.random() * names.length)] + Math.floor(Math.random() * 100),
      gender: genders[Math.floor(Math.random() * genders.length)],
      birth_date: `19${60 + Math.floor(Math.random() * 40)}-${String(Math.floor(Math.random() * 12) + 1).padStart(2, '0')}-${String(Math.floor(Math.random() * 28) + 1).padStart(2, '0')}`,
      phone: `1${3 + Math.floor(Math.random() * 6)}${String(Math.floor(Math.random() * 100000000)).padStart(9, '0')}`,
      nationality: nationalities[Math.floor(Math.random() * nationalities.length)],
      source_channel: channels[Math.floor(Math.random() * channels.length)],
      first_time_to_japan: Math.random() > 0.5,
      japanese_level: japaneseLevels[Math.floor(Math.random() * japaneseLevels.length)],
      accompany_count: Math.floor(Math.random() * 5),
      case_no: `CASE${Date.now().toString().slice(-6)}`,
      first_visit_date: new Date().toISOString().split('T')[0],
      allergy_tags: Math.random() > 0.7 ? (Math.random() > 0.5 ? ['青霉素'] : ['虾', '花生']) : [],
      chronic_disease_tags: Math.random() > 0.6 ? (Math.random() > 0.5 ? ['高血压'] : ['糖尿病', '高血脂']) : [],
      force_create: true,
    });

    const systolic = 100 + Math.floor(Math.random() * 60);
    const diastolic = 60 + Math.floor(Math.random() * 40);
    const height = 155 + Math.floor(Math.random() * 25);
    const weight = 50 + Math.floor(Math.random() * 40);

    await api.createMedicalRecord(patient.tsid, {
      chief_complaint: complaints[Math.floor(Math.random() * complaints.length)],
      systolic_bp: systolic,
      diastolic_bp: diastolic,
      temperature: 36 + Math.random() * 2,
      height,
      weight,
      icd10_code: `C${Math.floor(Math.random() * 80)}.${Math.floor(Math.random() * 9)}`,
      diagnosis: '初步诊断为需要进一步检查确定治疗方案',
      first_diagnosis_date: new Date().toISOString().split('T')[0],
      final_diagnosis: '待日本医院进一步确诊',
      case_type: caseTypes[Math.floor(Math.random() * caseTypes.length)],
      staging: ['I期', 'II期', 'III期', 'IV期'][Math.floor(Math.random() * 4)],
      second_opinion: Math.random() > 0.6,
      referral_hospital: hospitals[Math.floor(Math.random() * hospitals.length)],
      department: departments[Math.floor(Math.random() * departments.length)],
      attending_doctor: doctors[Math.floor(Math.random() * doctors.length)],
      treatment_plan: treatments[Math.floor(Math.random() * treatments.length)],
      treatment_cycle: `${Math.floor(Math.random() * 12) + 1}个疗程`,
      hospitalization: Math.random() > 0.5,
      hospital_days: Math.random() > 0.5 ? Math.floor(Math.random() * 30) + 1 : undefined,
      followup_status: followupStatuses[Math.floor(Math.random() * followupStatuses.length)],
      current_status: '治疗中',
    });

    page.value = 1;
    await loadPatients();
  } catch (e) {
    console.error('Mock create failed', e);
  } finally {
    mockLoading.value = false;
  }
}

onMounted(loadPatients);
</script>

<template>
  <div class="min-h-screen bg-base p-6">
    <div class="max-w-6xl mx-auto space-y-4">
      <!-- Header -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <h1 class="text-2xl font-semibold text-accent-secondary">患者列表</h1>
        <div class="flex flex-wrap gap-2">
          <button
            @click="viewMode = 'cards'"
            class="tab-btn"
            :class="{ active: viewMode === 'cards' }"
          >卡片</button>
          <button
            @click="viewMode = 'table'"
            class="tab-btn"
            :class="{ active: viewMode === 'table' }"
          >列表</button>
          <button
            @click="createMockPatient"
            :disabled="mockLoading"
            class="btn-primary"
          >
            {{ mockLoading ? '添加中...' : '+ Mock数据' }}
          </button>
          <button
            @click="router.push('/register')"
            class="btn-primary"
          >
            + 新建患者
          </button>
        </div>
      </div>

      <!-- Search -->
      <div class="flex flex-col sm:flex-row gap-2">
        <input
          v-model="search"
          @keyup.enter="page = 1; loadPatients()"
          placeholder="搜索姓名、TSID 或案例编号..."
          class="medical-input flex-1"
        />
        <button
          @click="page = 1; loadPatients()"
          class="btn-secondary"
        >
          <Icon name="search-line" size="base" extraClass="mr-1" />搜索
        </button>
      </div>

      <!-- Loading -->
      <div v-if="loading" class="text-center py-8 text-text-secondary">加载中...</div>

      <!-- Card view -->
      <template v-else-if="viewMode === 'cards'">
        <div v-if="patients.length === 0" class="medical-card text-center py-16 text-text-tertiary">暂无患者</div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div
            v-for="p in patients"
            :key="p.tsid"
            class="medical-card p-5"
          >
            <div class="flex items-start justify-between mb-3">
              <div>
                <h3 class="text-base font-semibold text-foreground">{{ p.name }}</h3>
                <p class="text-xs text-text-tertiary mt-0.5 font-mono">{{ p.tsid }}</p>
              </div>
              <div class="flex items-center gap-2">
                <span class="gender-pill" :class="p.gender === '男' ? 'gender-pill--male' : 'gender-pill--female'">{{ p.gender }}</span>
                <button
                  @click="router.push(`/patients/${p.tsid}/view`)"
                  class="text-text-tertiary hover:text-accent-primary p-1 transition-colors"
                  title="全屏视图"
                >
                  <Icon name="fullscreen-line" size="base" />
                </button>
              </div>
            </div>
            <div class="space-y-1.5 text-sm">
              <div class="flex justify-between text-text-secondary">
                <span>案例编号</span>
                <span class="text-foreground font-mono text-xs">{{ p.case_no ?? '—' }}</span>
              </div>
              <div class="flex justify-between text-text-secondary">
                <span>国籍</span>
                <span class="text-foreground">{{ p.nationality ?? '—' }}</span>
              </div>
              <div class="flex justify-between text-text-secondary">
                <span>来源渠道</span>
                <span class="text-foreground">{{ p.source_channel ?? '—' }}</span>
              </div>
              <div class="flex justify-between text-text-secondary">
                <span>首次赴日</span>
                <span class="text-foreground">{{ p.first_time_to_japan ? '是' : p.first_time_to_japan === false ? '否' : '—' }}</span>
              </div>
              <div class="flex justify-between text-text-secondary">
                <span>日语能力</span>
                <span class="text-foreground">{{ p.japanese_level ?? '—' }}</span>
              </div>
              <div class="flex justify-between text-text-secondary">
                <span>接诊时间</span>
                <span class="text-foreground">{{ p.first_visit_date ? formatDate(p.first_visit_date) : '—' }}</span>
              </div>
            </div>
            <div v-if="p.allergy_tags.length > 0 || p.chronic_disease_tags.length > 0" class="flex flex-wrap gap-1 mt-3 pt-3 border-t border-subtle">
              <span v-for="tag in p.allergy_tags" :key="tag" class="status-pill status-pill--red">{{ tag }}</span>
              <span v-for="tag in p.chronic_disease_tags" :key="tag" class="status-pill status-pill--orange">{{ tag }}</span>
            </div>
            <div class="mt-3 pt-3 border-t border-subtle flex justify-end">
              <button
                @click="router.push(`/patients/${p.tsid}`)"
                class="text-sm text-accent-secondary hover:text-accent-primary flex items-center gap-1"
              >
                查看详情<Icon name="arrow-right-s-line" size="base" />
              </button>
            </div>
          </div>
        </div>
      </template>

      <!-- Table view -->
      <div v-else class="medical-card overflow-x-auto">
        <table class="w-full text-sm min-w-[640px]">
          <thead class="bg-surface border-b border-subtle">
            <tr>
              <th class="px-4 py-3 text-left text-text-secondary font-medium">案例编号</th>
              <th class="px-4 py-3 text-left text-text-secondary font-medium">姓名</th>
              <th class="px-4 py-3 text-left text-text-secondary font-medium">性别</th>
              <th class="px-4 py-3 text-left text-text-secondary font-medium">出生日期</th>
              <th class="px-4 py-3 text-left text-text-secondary font-medium">国籍</th>
              <th class="px-4 py-3 text-left text-text-secondary font-medium">来源</th>
              <th class="px-4 py-3 text-left text-text-secondary font-medium">联系电话</th>
              <th class="px-4 py-3 text-left text-text-secondary font-medium">操作</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-subtle">
            <tr
              v-for="p in patients"
              :key="p.tsid"
              class="hover:bg-surface cursor-pointer transition-colors"
              @click="router.push(`/patients/${p.tsid}`)"
            >
              <td class="px-4 py-3 font-mono text-xs text-text-secondary">{{ p.case_no ?? '—' }}</td>
              <td class="px-4 py-3 font-medium text-foreground">{{ p.name }}</td>
              <td class="px-4 py-3 text-text-secondary">{{ p.gender }}</td>
              <td class="px-4 py-3 text-text-secondary">{{ formatDate(p.birth_date) }}</td>
              <td class="px-4 py-3 text-text-secondary">{{ p.nationality ?? '—' }}</td>
              <td class="px-4 py-3 text-text-tertiary text-xs">{{ p.source_channel ?? '—' }}</td>
              <td class="px-4 py-3 text-text-tertiary text-xs">{{ p.phone ?? '—' }}</td>
              <td class="px-4 py-3">
                <button
                  @click.stop="router.push(`/patients/${p.tsid}`)"
                  class="text-accent-secondary hover:text-accent-secondary text-sm"
                >
                  查看
                </button>
              </td>
            </tr>
            <tr v-if="patients.length === 0">
              <td colspan="8" class="px-4 py-8 text-center text-text-tertiary">暂无患者</td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div class="flex items-center justify-between text-sm text-text-secondary">
        <span>共 {{ total }} 条</span>
        <div class="flex gap-2">
          <button
            :disabled="page <= 1"
            @click="page--; loadPatients()"
            class="btn-secondary"
          >
            <Icon name="arrow-left-s-line" size="base" extraClass="mr-1" />上一页
          </button>
          <span class="px-3 py-2 text-foreground">{{ page }} / {{ Math.ceil(total / 20) || 1 }}</span>
          <button
            :disabled="patients.length < 20"
            @click="page++; loadPatients()"
            class="btn-secondary"
          >
            下一页<Icon name="arrow-right-s-line" size="base" extraClass="ml-1" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>