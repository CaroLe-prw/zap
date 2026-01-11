<template>
  <div class="app-container">
    <!-- 任务视图 -->
    <div v-if="currentView === 'tasks'" class="view-panel">
      <header class="panel-header">
        <div class="header-top">
          <h1 class="app-title">Zap</h1>
          <NButton
            quaternary
            circle
            size="small"
            @click="currentView = 'stats'"
          >
            <template #icon>
              <NIcon><BarChart /></NIcon>
            </template>
          </NButton>
        </div>
        <div class="tabs">
          <span
            v-for="tab in tabs"
            :key="tab"
            class="tab"
            :class="{ active: currentTab === tab }"
            @click="currentTab = tab"
          >
            {{ tab }}
          </span>
          <NButton
            v-if="showAddButton"
            quaternary
            size="small"
            class="add-task-btn"
            @click="showModal = true"
          >
            <template #icon>
              <NIcon><Add /></NIcon>
            </template>
            Add Task
          </NButton>
        </div>
      </header>

      <TodayFocus />

      <AddTaskModal
        v-model:show="showModal"
        @close="showModal = false"
        @add="handleAddTask"
        @add-start="handleAddTaskStart"
      />

      <div class="tasks-section">
        <div class="section-header">ALL TASKS</div>
        <div class="task-list">
          <TaskItem
            v-for="task in filteredTasks"
            :key="task.id"
            :task="task"
            :tag-color="tagColor"
            :format-time="formatTime"
            @toggle="toggleTimer"
          />
        </div>
      </div>

      <footer class="panel-footer">
        <span>Total: {{ formatTime(totalTime) }}</span>
        <span>This week: {{ formatTime(weekTime) }}</span>
      </footer>
    </div>

    <!-- 统计视图 -->
    <div v-else class="view-panel">
      <header class="panel-header">
        <div class="header-top">
          <h1 class="app-title">Zap</h1>
          <NButton
            quaternary
            circle
            size="small"
            @click="currentView = 'tasks'"
          >
            <template #icon>
              <NIcon><List /></NIcon>
            </template>
          </NButton>
        </div>
        <div class="stats-period">
          <span class="period-label">TODAY</span>
          <div class="period-nav">
            <span class="nav-btn">&lt;</span>
            <span class="nav-btn">&gt;</span>
          </div>
        </div>
      </header>
      <div class="stats-content">
        <div class="stats-placeholder">Statistics coming soon...</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { NButton, NIcon } from "naive-ui";
import { BarChart, List, Add } from "@vicons/ionicons5";
import TodayFocus from "./components/TodayFocus.vue";
import TaskItem from "./components/TaskItem.vue";
import AddTaskModal from "./components/AddTaskModal.vue";

const currentView = ref<"tasks" | "stats">("tasks");
const tabs = ["All Tasks", "In Progress", "Completed"];
const currentTab = ref("All Tasks");
const showModal = ref(false);

interface Task {
  id: number;
  title: string;
  category: string;
  completed: boolean;
  elapsed: number;
  sessionTime: number;
  isTracking: boolean;
}

const tasks = ref<Task[]>([
  {
    id: 1,
    title: "Review PR #234",
    category: "Code Review",
    completed: false,
    elapsed: 5025,
    sessionTime: 5025,
    isTracking: true,
  },
  {
    id: 2,
    title: "Design system update",
    category: "Design",
    completed: false,
    elapsed: 0,
    sessionTime: 0,
    isTracking: false,
  },
  {
    id: 3,
    title: "Team sync meeting",
    category: "Meeting",
    completed: false,
    elapsed: 0,
    sessionTime: 0,
    isTracking: false,
  },
  {
    id: 4,
    title: "Fix authentication bug",
    category: "Work",
    completed: true,
    elapsed: 3600,
    sessionTime: 0,
    isTracking: false,
  },
]);

const filteredTasks = computed(() => {
  if (currentTab.value === "All Tasks") return tasks.value;
  if (currentTab.value === "In Progress")
    return tasks.value.filter((t) => !t.completed);
  return tasks.value.filter((t) => t.completed);
});

const totalTime = computed(() =>
  tasks.value.reduce((sum, t) => sum + t.elapsed, 0),
);
const weekTime = ref(28800);

const showAddButton = computed(() => currentView.value === "tasks");

const handleAddTask = (data: { form: any; addToFocus: boolean }) => {
  const category = data.form.category || "Other";
  const newTask: Task = {
    id: Date.now(),
    title: data.form.taskName,
    category,
    completed: false,
    elapsed: 0,
    sessionTime: 0,
    isTracking: false,
  };
  tasks.value.unshift(newTask);
};

const handleAddTaskStart = (data: { form: any; addToFocus: boolean }) => {
  // Pause all other tracking tasks
  tasks.value.forEach((t) => {
    if (t.isTracking) {
      t.elapsed += t.sessionTime;
      t.sessionTime = 0;
      t.isTracking = false;
    }
  });

  const category = data.form.category || "Other";
  const newTask: Task = {
    id: Date.now(),
    title: data.form.taskName,
    category,
    completed: false,
    elapsed: 0,
    sessionTime: 0,
    isTracking: true,
  };
  tasks.value.unshift(newTask);
  currentTab.value = "In Progress";
};

const tagColor = (category: string) => {
  const colors: Record<string, { color: string; textColor: string }> = {
    Work: { color: "#3B82F6", textColor: "#fff" },
    Study: { color: "#A855F7", textColor: "#fff" },
    Life: { color: "#22C55E", textColor: "#fff" },
    Health: { color: "#06B6D4", textColor: "#fff" },
    Meeting: { color: "#F97316", textColor: "#fff" },
    Other: { color: "#9CA3AF", textColor: "#fff" },
  };
  return colors[category] || { color: "#9CA3AF", textColor: "#fff" };
};

const formatTime = (seconds: number) => {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return `${h.toString().padStart(2, "0")}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
};

const toggleTimer = (task: Task) => {
  if (task.isTracking) {
    // 暂停时保存时间
    task.elapsed += task.sessionTime;
    task.sessionTime = 0;
    task.isTracking = false;
  } else {
    // 开始时：先保存其他正在计时的任务，再重置当前任务
    tasks.value.forEach((t) => {
      if (t.isTracking) {
        t.elapsed += t.sessionTime;
        t.sessionTime = 0;
        t.isTracking = false;
      }
    });
    task.sessionTime = 0;
    task.isTracking = true;
  }
};

// 计时器
let timer: ReturnType<typeof setInterval> | null = null;

const startTimer = () => {
  timer = setInterval(() => {
    tasks.value.forEach((task) => {
      if (task.isTracking) {
        task.sessionTime++;
      }
    });
  }, 1000);
};

const stopTimer = () => {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
};

onMounted(() => {
  startTimer();
});

onUnmounted(() => {
  stopTimer();
});
</script>

<style scoped>
.app-container {
  width: 100%;
  height: 100vh;
  background: white;
}

.view-panel {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 32px;
}

.panel-header {
  margin-bottom: 20px;
}

.header-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.app-title {
  font-size: 28px;
  font-weight: 700;
  color: #111827;
  margin: 0;
}

.tabs {
  display: flex;
  align-items: center;
  gap: 20px;
  border-bottom: 2px solid #e5e7eb;
  padding-bottom: 0;
}

.add-task-btn {
  margin-left: auto;
  font-weight: 600;
  color: #111827;
}

.tab {
  font-size: 13px;
  font-weight: 500;
  color: #9ca3af;
  padding-bottom: 8px;
  cursor: pointer;
  position: relative;
  bottom: -2px;
  border-bottom: 2px solid transparent;
  margin-bottom: -2px;
}

.tab.active {
  color: #111827;
  border-bottom-color: #111827;
}

.section-header {
  font-size: 11px;
  font-weight: 600;
  color: #9ca3af;
  letter-spacing: 1px;
  margin: 20px 0 12px;
}

.task-list {
  flex: 1;
  overflow-y: auto;
  padding-bottom: 16px;
}

.panel-footer {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #6b7280;
  padding-top: 16px;
  border-top: 1px solid #e5e7eb;
  margin-top: auto;
}

.stats-period {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 20px;
}

.period-label {
  font-size: 13px;
  font-weight: 600;
  color: #111827;
}

.period-nav {
  display: flex;
  gap: 8px;
}

.nav-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f3f4f6;
  border-radius: 6px;
  cursor: pointer;
  font-size: 12px;
}

.stats-content {
  flex: 1;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 40px;
}

.stats-placeholder {
  color: #9ca3af;
}
</style>
