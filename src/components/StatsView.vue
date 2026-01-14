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
      <button class="export-btn">
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
          <div class="card-value">2h 45m</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Focused Time</div>
          <div class="card-value highlight">2h 00m</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Sessions</div>
          <div class="card-value">4</div>
        </div>
      </div>

      <!-- Category Chart -->
      <div class="chart-section">
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
              <div class="center-time">1h 25m</div>
              <div class="center-label">Work</div>
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
              <span class="legend-time">{{ cat.time }}</span>
              <span class="legend-percent">{{ cat.percentage }}%</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Detailed Report -->
      <div class="report-section">
        <div class="section-title">Detailed Report</div>
        <div class="report-list">
          <div v-for="item in todayReport" :key="item.task" class="report-item">
            <div class="report-main">
              <span
                class="report-dot"
                :style="{ background: item.color }"
              ></span>
              <span class="report-task">{{ item.task }}</span>
              <NTag
                size="small"
                :bordered="false"
                :color="getTagColor(item.category)"
              >
                {{ item.category }}
              </NTag>
            </div>
            <div class="report-meta">
              <span class="report-time">{{ item.time }}</span>
              <span class="report-last">{{ item.last }}</span>
            </div>
          </div>
        </div>
        <div class="report-footer">No more entries today</div>
      </div>
    </template>

    <!-- Week View -->
    <template v-else-if="currentPeriod === 'week'">
      <!-- Summary Cards -->
      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-label">Total This Week</div>
          <div class="card-value">28h 45m</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Daily Average</div>
          <div class="card-value highlight">4h 06m</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Sessions</div>
          <div class="card-value">32</div>
        </div>
      </div>

      <!-- Weekly Bar Chart -->
      <div class="chart-section">
        <div class="section-title">Daily Breakdown</div>
        <div class="bar-chart">
          <div v-for="day in weekData" :key="day.name" class="bar-item">
            <div class="bar-track">
              <div
                class="bar-fill"
                :style="{ height: day.percentage + '%', background: day.color }"
              ></div>
            </div>
            <div class="bar-label">{{ day.name }}</div>
            <div class="bar-value">{{ day.time }}</div>
          </div>
        </div>
      </div>

      <!-- Week Categories -->
      <div class="chart-section">
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
              <span class="category-time">{{ cat.time }}</span>
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
    </template>

    <!-- Month View -->
    <template v-else-if="currentPeriod === 'month'">
      <!-- Summary Cards -->
      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-label">Total This Month</div>
          <div class="card-value">126h 30m</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Daily Average</div>
          <div class="card-value highlight">4h 21m</div>
        </div>
        <div class="summary-card">
          <div class="card-label">Active Days</div>
          <div class="card-value">29</div>
        </div>
      </div>

      <!-- Monthly Overview -->
      <div class="chart-section">
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
            :title="day.date + ': ' + day.time"
          >
            <span class="day-tooltip">{{ day.time }}</span>
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
      <div class="chart-section">
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
              <span class="category-time">{{ cat.time }}</span>
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
      <div class="report-section">
        <div class="section-title">Top Tasks</div>
        <div class="report-list">
          <div
            v-for="(item, index) in topTasks"
            :key="item.task"
            class="report-item"
          >
            <div class="report-main">
              <span class="rank">{{ index + 1 }}</span>
              <span class="report-task">{{ item.task }}</span>
              <NTag
                size="small"
                :bordered="false"
                :color="getTagColor(item.category)"
              >
                {{ item.category }}
              </NTag>
            </div>
            <div class="report-time">{{ item.time }}</div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { NTag, NDatePicker } from "naive-ui";

defineEmits<{
  (e: "back"): void;
}>();

const periods = [
  { label: "Today", value: "today" },
  { label: "Week", value: "week" },
  { label: "Month", value: "month" },
] as const;

const currentPeriod = ref<(typeof periods)[number]["value"]>("today");

const today = new Date();
// Today: 当前日期零点
const todayStart = new Date(
  today.getFullYear(),
  today.getMonth(),
  today.getDate(),
).getTime();
// Week: 当前日期 + 7天范围
const weekEnd = new Date(
  today.getFullYear(),
  today.getMonth(),
  today.getDate() + 6,
).getTime();
// Month: 当前年当前月
const monthValue = new Date(today.getFullYear(), today.getMonth(), 1).getTime();

// 根据 period 设置正确的默认值
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

// 切换 period 时更新日期值
watch(currentPeriod, () => {
  dateValue.value = getDefaultValue();
});

// Week 模式：自动调整为 7 天范围
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

// Today data
const todayCategories = [
  { name: "Work", time: "1h 25m", percentage: 51.1, color: "#3b82f6" },
  { name: "Study", time: "30m", percentage: 18.2, color: "#a855f7" },
  { name: "Life", time: "30m", percentage: 18.2, color: "#22c55e" },
  { name: "Health", time: "15m", percentage: 7.3, color: "#06b6d4" },
  { name: "Other", time: "9m", percentage: 5.2, color: "#9ca3af" },
];

const todayReport = [
  {
    task: "Prepare presentation",
    category: "Work",
    time: "1h 25m",
    last: "9:55 am",
    color: "#3b82f6",
  },
  {
    task: "Write report",
    category: "Meeting",
    time: "30m",
    last: "10:35 am",
    color: "#f97316",
  },
  {
    task: "Lunch with friend",
    category: "Life",
    time: "30m",
    last: "12:15 pm",
    color: "#22c55e",
  },
  {
    task: "Gym workout",
    category: "Health",
    time: "15m",
    last: "13:35 pm",
    color: "#06b6d4",
  },
  {
    task: "Read book",
    category: "Study",
    time: "5m",
    last: "15:40 pm",
    color: "#a855f7",
  },
];

// Week data
const weekData = [
  { name: "Mon", time: "4h 20m", percentage: 100, color: "#3b82f6" },
  { name: "Tue", time: "3h 45m", percentage: 87, color: "#3b82f6" },
  { name: "Wed", time: "5h 10m", percentage: 100, color: "#3b82f6" },
  { name: "Thu", time: "4h 30m", percentage: 100, color: "#3b82f6" },
  { name: "Fri", time: "3h 50m", percentage: 89, color: "#3b82f6" },
  { name: "Sat", time: "2h 40m", percentage: 62, color: "#64748b" },
  { name: "Sun", time: "4h 30m", percentage: 100, color: "#64748b" },
];

const weekCategories = [
  { name: "Work", time: "18h 30m", percentage: 64.3, color: "#3b82f6" },
  { name: "Study", time: "4h 15m", percentage: 14.8, color: "#a855f7" },
  { name: "Meeting", time: "3h 00m", percentage: 10.4, color: "#f97316" },
  { name: "Life", time: "2h 00m", percentage: 6.9, color: "#22c55e" },
  { name: "Health", time: "1h 00m", percentage: 3.5, color: "#06b6d4" },
];

// Month data
interface MonthDay {
  date: string;
  time: string;
  active: boolean;
  level: number;
}

const monthData: MonthDay[] = Array.from({ length: 31 }, (_, i) => ({
  date: `01-${(i + 1).toString().padStart(2, "0")}`,
  time:
    i % 3 === 0
      ? "0m"
      : `${Math.floor(Math.random() * 6) + 1}h ${Math.floor(Math.random() * 60)}m`,
  active: i % 3 !== 0,
  level: i % 4 === 0 ? 0 : i % 4 === 1 ? 1 : i % 4 === 2 ? 2 : 3,
}));

const monthCategories = [
  { name: "Work", time: "82h 30m", percentage: 65.2, color: "#3b82f6" },
  { name: "Study", time: "18h 45m", percentage: 14.8, color: "#a855f7" },
  { name: "Meeting", time: "12h 00m", percentage: 9.5, color: "#f97316" },
  { name: "Life", time: "8h 15m", percentage: 6.5, color: "#22c55e" },
  { name: "Health", time: "5h 00m", percentage: 4.0, color: "#06b6d4" },
];

const topTasks = [
  { task: "Code review and PR merge", category: "Work", time: "12h 30m" },
  { task: "Feature development", category: "Work", time: "24h 15m" },
  { task: "Documentation updates", category: "Work", time: "8h 45m" },
  { task: "Team sync meetings", category: "Meeting", time: "6h 00m" },
  { task: "Learning new technology", category: "Study", time: "10h 30m" },
];

const circumference = 2 * Math.PI * 40;

const getStrokeDasharray = (percentage: number) => {
  const length = (percentage / 100) * circumference;
  return `${length} ${circumference - length}`;
};

const getStrokeDashoffset = (index: number, data: typeof todayCategories) => {
  let offset = 0;
  for (let i = 0; i < index; i++) {
    const cat = data[i];
    if (cat) {
      offset += (cat.percentage / 100) * circumference;
    }
  }
  return -offset;
};

const getTagColor = (category: string) => {
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

.export-btn:hover {
  background: var(--bg-page);
  color: var(--text-primary);
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
</style>
