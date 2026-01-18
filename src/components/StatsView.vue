<template>
  <div class="stats-view">
    <!-- Header -->
    <header class="stats-header">
      <button class="back-btn" @click="$emit('back')">
        <svg
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M19 12H5M12 19l-7-7 7-7" />
        </svg>
      </button>
      <h1 class="stats-title">Statistics</h1>
      <button class="export-btn" disabled>
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3"
          />
        </svg>
        Export
      </button>
    </header>

    <!-- Time Filter -->
    <div class="time-filter">
      <div class="filter-tabs">
        <button
          v-for="period in periods"
          :key="period.value"
          class="filter-tab"
          :class="{ active: currentPeriod === period.value }"
          @click="currentPeriod = period.value"
        >
          {{ period.label }}
        </button>
      </div>
      <NDatePicker
        v-model:value="dateValue"
        :type="datePickerType"
        :clearable="false"
        size="small"
        class="date-picker-input"
        placement="bottom-end"
        :style="{ width: datePickerWidth }"
        @update:value="handleDateChange"
      />
    </div>

    <!-- Today View -->
    <template v-if="currentPeriod === 'today'">
      <!-- Summary Cards -->
      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-label">Total Today</div>
          <div class="card-value">{{ formatCardTime(todayTotalTime) }}</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Focused Time</div>
          <div class="card-value highlight">
            {{ formatCardTime(todayFocusedTime) }}
          </div>
        </div>
        <div class="summary-card">
          <div class="card-label">Sessions</div>
          <div class="card-value">{{ todaySessions }}</div>
        </div>
      </div>

      <!-- Category Chart -->
      <div v-if="todayCategories.length > 0" class="chart-section">
        <div class="section-title">Time by Category</div>
        <div class="chart-container">
          <div class="donut-chart">
            <svg viewBox="0 0 100 100" class="chart-svg">
              <circle
                v-for="(cat, index) in todayCategories"
                :key="cat.name"
                cx="50"
                cy="50"
                r="40"
                fill="none"
                :stroke="cat.color"
                stroke-width="20"
                :stroke-dasharray="getStrokeDasharray(cat.percentage)"
                :stroke-dashoffset="getStrokeDashoffset(index, todayCategories)"
                class="chart-segment"
              />
            </svg>
            <div class="chart-center">
              <div class="center-time">
                {{ formatTime(topCategory?.seconds || 0) }}
              </div>
              <div class="center-label">{{ topCategory?.name || "-" }}</div>
            </div>
          </div>
          <div class="chart-legend">
            <div
              v-for="cat in todayCategories"
              :key="cat.name"
              class="legend-item"
            >
              <span
                class="legend-dot"
                :style="{ background: cat.color }"
              ></span>
              <span class="legend-name">{{ cat.name }}</span>
              <span class="legend-time">{{ formatTime(cat.seconds) }}</span>
              <span class="legend-percent">{{ cat.percentage }}%</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Detailed Report -->
      <div v-if="todayReport.length > 0" class="report-section">
        <div class="section-title">Detailed Report</div>
        <div class="report-list">
          <div
            v-for="item in todayReport"
            :key="item.task_id"
            class="report-item"
          >
            <div class="report-main">
              <span
                class="report-dot"
                :style="{ background: item.category_color || '#9ca3af' }"
              ></span>
              <span class="report-task">{{ item.task_title }}</span>
              <NTag
                size="small"
                :bordered="false"
                :color="getTagColor(item.category)"
              >
                {{ item.category || "Other" }}
              </NTag>
            </div>
            <div class="report-meta">
              <span class="report-time">{{ formatTime(item.seconds) }}</span>
              <span class="report-last">{{
                formatCompletedTime(item.last_time) || "-"
              }}</span>
            </div>
          </div>
        </div>
      </div>
      <!-- Empty state -->
      <div v-else class="empty-state">
        <p>No time tracked today</p>
      </div>
    </template>

    <!-- Week View -->
    <template v-else-if="currentPeriod === 'week'">
      <!-- Summary Cards -->
      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-label">Total This Week</div>
          <div class="card-value">{{ formatCardTime(weekTotalTime) }}</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Daily Average</div>
          <div class="card-value highlight">
            {{ formatCardTime(weekDailyAvg) }}
          </div>
        </div>
        <div class="summary-card">
          <div class="card-label">Sessions</div>
          <div class="card-value">{{ weekSessions }}</div>
        </div>
      </div>

      <!-- Weekly Bar Chart -->
      <div v-if="weekData.length > 0" class="chart-section">
        <div class="section-title">Daily Breakdown</div>
        <div class="bar-chart">
          <div v-for="day in weekData" :key="day.date" class="bar-item">
            <div class="bar-track">
              <div
                class="bar-fill"
                :style="{
                  height: day.percentage + '%',
                  background: day.percentage > 0 ? '#3b82f6' : '#64748b',
                }"
              ></div>
            </div>
            <div class="bar-label">{{ day.day_name }}</div>
            <div class="bar-value">{{ formatTime(day.seconds) }}</div>
          </div>
        </div>
      </div>

      <!-- Week Categories -->
      <div v-if="weekCategories.length > 0" class="chart-section">
        <div class="section-title">Time by Category</div>
        <div class="category-list">
          <div
            v-for="cat in weekCategories"
            :key="cat.name"
            class="category-item"
          >
            <div class="category-left">
              <span
                class="category-dot"
                :style="{ background: cat.color }"
              ></span>
              <span class="category-name">{{ cat.name }}</span>
            </div>
            <div class="category-right">
              <span class="category-time">{{ formatTime(cat.seconds) }}</span>
              <span class="category-percent">{{ cat.percentage }}%</span>
            </div>
            <div class="category-bar">
              <div
                class="category-bar-fill"
                :style="{ width: cat.percentage + '%', background: cat.color }"
              ></div>
            </div>
          </div>
        </div>
      </div>
      <!-- Empty state -->
      <div v-if="weekData.length === 0" class="empty-state">
        <p>No time tracked this week</p>
      </div>
    </template>

    <!-- Month View -->
    <template v-else-if="currentPeriod === 'month'">
      <!-- Summary Cards -->
      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-label">Total This Month</div>
          <div class="card-value">{{ formatCardTime(monthTotalTime) }}</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Daily Average</div>
          <div class="card-value highlight">
            {{ formatCardTime(monthDailyAvg) }}
          </div>
        </div>
        <div class="summary-card">
          <div class="card-label">Active Days</div>
          <div class="card-value">{{ monthActiveDays }}</div>
        </div>
      </div>

      <!-- Monthly Overview -->
      <div v-if="monthData.length > 0" class="chart-section">
        <div class="section-title">Monthly Overview</div>
        <div class="month-grid">
          <div
            v-for="day in monthData"
            :key="day.date"
            class="month-day"
            :class="{
              active: day.active,
              low: day.level === 1,
              medium: day.level === 2,
              high: day.level === 3,
            }"
            :title="day.date + ': ' + formatTime(day.seconds)"
          >
            <span class="day-tooltip">{{ formatTime(day.seconds) }}</span>
          </div>
        </div>
        <div class="month-legend">
          <span class="legend-text">Less</span>
          <span class="legend-box empty"></span>
          <span class="legend-box low"></span>
          <span class="legend-box medium"></span>
          <span class="legend-box high"></span>
          <span class="legend-text">More</span>
        </div>
      </div>

      <!-- Month Categories -->
      <div v-if="monthCategories.length > 0" class="chart-section">
        <div class="section-title">Time by Category</div>
        <div class="category-list">
          <div
            v-for="cat in monthCategories"
            :key="cat.name"
            class="category-item"
          >
            <div class="category-left">
              <span
                class="category-dot"
                :style="{ background: cat.color }"
              ></span>
              <span class="category-name">{{ cat.name }}</span>
            </div>
            <div class="category-right">
              <span class="category-time">{{ formatTime(cat.seconds) }}</span>
              <span class="category-percent">{{ cat.percentage }}%</span>
            </div>
            <div class="category-bar">
              <div
                class="category-bar-fill"
                :style="{ width: cat.percentage + '%', background: cat.color }"
              ></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Top Tasks -->
      <div v-if="topTasks.length > 0" class="report-section">
        <div class="section-title">Top Tasks</div>
        <div class="report-list">
          <div
            v-for="(item, index) in topTasks"
            :key="item.task_id"
            class="report-item"
          >
            <div class="report-main">
              <span class="rank">{{ index + 1 }}</span>
              <span class="report-task">{{ item.task_title }}</span>
              <NTag
                size="small"
                :bordered="false"
                :color="getTagColor(item.category)"
              >
                {{ item.category || "Other" }}
              </NTag>
            </div>
            <div class="report-time">{{ formatTime(item.seconds) }}</div>
          </div>
        </div>
      </div>
      <!-- Empty state -->
      <div v-if="monthData.length === 0" class="empty-state">
        <p>No time tracked this month</p>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { formatCompletedTime } from "../utils/time";
import { NTag, NDatePicker } from "naive-ui";

defineEmits<{
  (e: "back"): void;
}>();

interface CategoryStat {
  name: string;
  color: string;
  seconds: number;
  percentage: number;
}

interface DailyStat {
  day_name: string;
  date: string;
  seconds: number;
  percentage: number;
}

interface MonthlyDailyStat {
  date: string;
  seconds: number;
  active: boolean;
  level: number;
}

interface TaskStat {
  task_id: number;
  task_title: string;
  category: string | null;
  category_color: string | null;
  seconds: number;
  last_time: string | null;
}

interface TodayStats {
  total_seconds: number;
  focused_seconds: number;
  sessions_count: number;
  categories: CategoryStat[];
  detailed_report: TaskStat[];
}

interface WeekStats {
  total_seconds: number;
  daily_average_seconds: number;
  sessions_count: number;
  daily_breakdown: DailyStat[];
  categories: CategoryStat[];
}

interface MonthStats {
  total_seconds: number;
  daily_average_seconds: number;
  active_days: number;
  monthly_overview: MonthlyDailyStat[];
  categories: CategoryStat[];
  top_tasks: TaskStat[];
}

const periods = [
  { label: "Today", value: "today" },
  { label: "Week", value: "week" },
  { label: "Month", value: "month" },
] as const;

const currentPeriod = ref<(typeof periods)[number]["value"]>("today");
const loading = ref(false);
const todayStats = ref<TodayStats | null>(null);
const weekStats = ref<WeekStats | null>(null);
const monthStats = ref<MonthStats | null>(null);

const today = new Date();

const todayStart = new Date(
  today.getFullYear(),
  today.getMonth(),
  today.getDate(),
).getTime();

const weekEnd = new Date(
  today.getFullYear(),
  today.getMonth(),
  today.getDate() + 6,
).getTime();

const monthValue = new Date(today.getFullYear(), today.getMonth(), 1).getTime();

const getDefaultValue = () => {
  if (currentPeriod.value === "today") return todayStart;
  if (currentPeriod.value === "week")
    return [todayStart, weekEnd] as [number, number];
  return monthValue;
};

const dateValue = ref<number | [number, number] | null>(getDefaultValue());

const datePickerType = computed(() => {
  if (currentPeriod.value === "today") return "date";
  if (currentPeriod.value === "week") return "daterange";
  return "month";
});

const datePickerWidth = computed(() => {
  if (currentPeriod.value === "today") return "140px";
  if (currentPeriod.value === "week") return "280px";
  return "140px";
});

// Format seconds to human readable time (e.g., "1h 25m", "30m", "6s")
const formatTime = (seconds: number): string => {
  if (seconds < 60) return `${seconds}s`;
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  if (hours === 0) return `${mins}m`;
  if (mins === 0) return `${hours}h`;
  return `${hours}h ${mins}m`;
};

// Format seconds to card value (e.g., "2h 45m")
const formatCardTime = (seconds: number): string => {
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  return `${hours}h ${mins}m`;
};

const fetchStats = async () => {
  loading.value = true;
  try {
    const buildQuery = (): { start_date?: string; end_date?: string } => {
      if (currentPeriod.value === "today") {
        if (typeof dateValue.value === "number") {
          const d = new Date(dateValue.value);
          // 使用本地时区格式化 YYYY-MM-DD
          const dateStr = d.toLocaleDateString("en-CA");
          return { start_date: dateStr };
        }
      } else if (currentPeriod.value === "week") {
        if (Array.isArray(dateValue.value) && dateValue.value.length >= 2) {
          const start = new Date(dateValue.value[0]).toLocaleDateString(
            "en-CA",
          );
          const end = new Date(dateValue.value[1]).toLocaleDateString("en-CA");
          return { start_date: start, end_date: end };
        }
      } else {
        // Month - dateValue is timestamp of first day of month
        if (typeof dateValue.value === "number") {
          const d = new Date(dateValue.value);
          const year = d.getFullYear();
          const month = String(d.getMonth() + 1).padStart(2, "0");
          const dateStr = `${year}-${month}`;
          return { start_date: dateStr };
        }
      }
      return {};
    };

    const query = buildQuery();

    if (currentPeriod.value === "today") {
      todayStats.value = await invoke("get_today_stats", { query });
    } else if (currentPeriod.value === "week") {
      weekStats.value = await invoke("get_week_stats", { query });
    } else {
      monthStats.value = await invoke("get_month_stats", { query });
    }
  } catch (error) {
    console.error("Failed to fetch stats:", error);
  } finally {
    loading.value = false;
  }
};

watch(currentPeriod, () => {
  dateValue.value = getDefaultValue();
  fetchStats();
});

watch(dateValue, () => {
  fetchStats();
});

onMounted(() => {
  fetchStats();
});

const handleDateChange = (value: number | [number, number] | null) => {
  if (
    currentPeriod.value === "week" &&
    Array.isArray(value) &&
    value.length >= 2
  ) {
    const [start, end] = value;
    const diffDays = Math.ceil((end - start) / (1000 * 60 * 60 * 24));
    if (diffDays > 7) {
      dateValue.value = [start, start + 6 * 24 * 60 * 60 * 1000] as [
        number,
        number,
      ];
    } else {
      dateValue.value = value;
    }
  } else {
    dateValue.value = value;
  }
};

// Computed data for template
const todayCategories = computed(() => todayStats.value?.categories || []);
const todayReport = computed(() => todayStats.value?.detailed_report || []);
const todayTotalTime = computed(() => todayStats.value?.total_seconds || 0);
const todayFocusedTime = computed(() => todayStats.value?.focused_seconds || 0);
const todaySessions = computed(() => todayStats.value?.sessions_count || 0);

const weekData = computed(() => weekStats.value?.daily_breakdown || []);
const weekCategories = computed(() => weekStats.value?.categories || []);
const weekTotalTime = computed(() => weekStats.value?.total_seconds || 0);
const weekDailyAvg = computed(
  () => weekStats.value?.daily_average_seconds || 0,
);
const weekSessions = computed(() => weekStats.value?.sessions_count || 0);

const monthData = computed(() => monthStats.value?.monthly_overview || []);
const monthCategories = computed(() => monthStats.value?.categories || []);
const topTasks = computed(() => monthStats.value?.top_tasks || []);
const monthTotalTime = computed(() => monthStats.value?.total_seconds || 0);
const monthDailyAvg = computed(
  () => monthStats.value?.daily_average_seconds || 0,
);
const monthActiveDays = computed(() => monthStats.value?.active_days || 0);

// Get the top category for donut chart center
const topCategory = computed(() => {
  if (!todayCategories.value.length) return null;
  return todayCategories.value[0];
});

const circumference = 2 * Math.PI * 40;

const getStrokeDasharray = (percentage: number) => {
  const length = (percentage / 100) * circumference;
  return `${length} ${circumference - length}`;
};

const getStrokeDashoffset = (index: number, data: CategoryStat[]) => {
  let offset = 0;
  for (let i = 0; i < index; i++) {
    const cat = data[i];
    if (cat) {
      offset += (cat.percentage / 100) * circumference;
    }
  }
  return -offset;
};

const getTagColor = (category: string | null) => {
  if (!category) return { color: "#f3f4f6", textColor: "#374151" };
  const colors: Record<string, { color: string; textColor: string }> = {
    Work: { color: "#dbeafe", textColor: "#1d4ed8" },
    Meeting: { color: "#fef3c7", textColor: "#b45309" },
    Life: { color: "#dcfce7", textColor: "#15803d" },
    Health: { color: "#cffafe", textColor: "#0e7490" },
    Study: { color: "#f3e8ff", textColor: "#7c3aed" },
  };
  return colors[category] || { color: "#f3f4f6", textColor: "#374151" };
};
</script>

<style scoped>
.stats-view {
  padding-bottom: 32px;
}

.stats-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.back-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 4px;
  transition: color 0.15s;
}

.back-btn:hover {
  color: var(--text-primary);
}

.stats-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.export-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  background: none;
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 12px;
  color: var(--text-secondary);
  padding: 6px 10px;
  cursor: pointer;
  transition: all 0.15s;
}

.export-btn[disabled] {
  opacity: 0;
  visibility: hidden;
}

.time-filter {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.filter-tabs {
  display: flex;
  background: var(--bg-page);
  border-radius: 8px;
  padding: 4px;
}

.filter-tab {
  background: none;
  border: none;
  font-size: 13px;
  color: var(--text-secondary);
  padding: 6px 14px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
}

.filter-tab.active {
  background: white;
  color: var(--primary);
  font-weight: 500;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.date-picker-input {
  width: 140px;
}

.date-picker-input :deep(.n-date-picker-editor) {
  padding-left: 8px;
}

.summary-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  margin-bottom: 24px;
}

.summary-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 16px;
  text-align: center;
}

.card-label {
  font-size: 12px;
  color: var(--text-muted);
  margin-bottom: 6px;
}

.card-value {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.card-value.highlight {
  color: var(--primary);
}

.section-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 14px;
}

/* Chart Section */
.chart-section {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 20px;
  margin-bottom: 20px;
}

.chart-container {
  display: flex;
  align-items: center;
  gap: 24px;
}

.donut-chart {
  position: relative;
  width: 140px;
  height: 140px;
  flex-shrink: 0;
}

.chart-svg {
  width: 100%;
  height: 100%;
  transform: rotate(-90deg);
}

.chart-segment {
  transition: opacity 0.2s;
}

.chart-center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
}

.center-time {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.center-label {
  font-size: 11px;
  color: var(--text-muted);
}

.chart-legend {
  flex: 1;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 0;
  font-size: 13px;
}

.legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.legend-name {
  color: var(--text-primary);
  flex: 1;
}

.legend-time {
  color: var(--text-secondary);
  font-variant-numeric: tabular-nums;
}

.legend-percent {
  color: var(--text-muted);
  font-size: 12px;
  width: 36px;
  text-align: right;
}

/* Bar Chart */
.bar-chart {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 8px;
  height: 120px;
  padding-top: 10px;
}

.bar-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
}

.bar-track {
  width: 100%;
  height: 80px;
  background: var(--bg-page);
  border-radius: 4px;
  display: flex;
  align-items: flex-end;
  overflow: hidden;
}

.bar-fill {
  width: 100%;
  border-radius: 4px;
  transition: height 0.3s ease;
}

.bar-label {
  font-size: 11px;
  color: var(--text-muted);
}

.bar-value {
  font-size: 11px;
  font-weight: 500;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

/* Category List */
.category-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.category-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.category-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.category-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.category-name {
  font-size: 13px;
  color: var(--text-primary);
}

.category-right {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.category-time {
  color: var(--text-secondary);
  font-variant-numeric: tabular-nums;
}

.category-percent {
  color: var(--text-muted);
}

.category-bar {
  height: 4px;
  background: var(--bg-page);
  border-radius: 2px;
  overflow: hidden;
}

.category-bar-fill {
  height: 100%;
  border-radius: 2px;
  transition: width 0.3s ease;
}

/* Month Grid */
.month-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 4px;
}

.month-day {
  aspect-ratio: 1;
  border-radius: 3px;
  background: var(--bg-page);
  position: relative;
  cursor: pointer;
}

.month-day.low {
  background: #dbeafe;
}

.month-day.medium {
  background: #93c5fd;
}

.month-day.high {
  background: #3b82f6;
}

.day-tooltip {
  display: none;
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  background: var(--text-primary);
  color: white;
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  white-space: nowrap;
  margin-bottom: 4px;
  z-index: 10;
}

.month-day:hover .day-tooltip {
  display: block;
}

.month-legend {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 4px;
  margin-top: 12px;
  font-size: 11px;
  color: var(--text-muted);
}

.legend-box {
  width: 12px;
  height: 12px;
  border-radius: 2px;
}

.legend-box.empty {
  background: var(--bg-page);
}

.legend-box.low {
  background: #dbeafe;
}

.legend-box.medium {
  background: #93c5fd;
}

.legend-box.high {
  background: #3b82f6;
}

.legend-text {
  margin: 0 4px;
}

/* Report Section */
.report-section {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  overflow: hidden;
  margin-top: 20px;
  padding: 16px 16px 0;
}

.report-list {
  padding: 0;
}

.report-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
  border-bottom: 1px solid var(--border);
}

.report-item:last-child {
  border-bottom: none;
}

.report-main {
  display: flex;
  align-items: center;
  gap: 10px;
}

.rank {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-page);
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
}

.report-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.report-task {
  font-size: 14px;
  color: var(--text-primary);
}

.report-meta {
  display: flex;
  align-items: center;
  gap: 16px;
}

.report-time {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.report-last {
  font-size: 12px;
  color: var(--text-muted);
}

.report-footer {
  text-align: center;
  padding: 16px;
  font-size: 12px;
  color: var(--text-muted);
  border-top: 1px solid var(--border);
  background: var(--bg-page);
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: var(--text-muted);
}
</style>
