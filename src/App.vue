<template>
  <div class="app">
    <!-- Task View -->
    <template v-if="currentView === 'tasks'">
      <header class="header">
        <h1 class="logo">Zap</h1>
        <nav class="nav">
          <button
            v-for="tab in tabs"
            :key="tab"
            class="nav-item"
            :class="{ active: currentTab === tab }"
            @click="currentTab = tab"
          >
            {{ tab }}
          </button>
          <button class="icon-btn" title="Statistics" @click="currentView = 'stats'">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M18 20V10M12 20V4M6 20v-6"/>
            </svg>
          </button>
          <button class="add-btn" @click="showModal = true">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <line x1="12" y1="5" x2="12" y2="19"/>
              <line x1="5" y1="12" x2="19" y2="12"/>
            </svg>
            Add Task
          </button>
        </nav>
      </header>

      <TodayFocus />

      <main class="main">
        <div class="section-header">
          <span>All Tasks</span>
          <span class="count">{{ tasks.length }}</span>
        </div>
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
      </main>

      <footer class="footer">
        <div class="stat">
          <span class="stat-label">Total</span>
          <span class="stat-value">{{ formatTime(totalTime) }}</span>
        </div>
        <div class="stat">
          <span class="stat-label">This week</span>
          <span class="stat-value">{{ formatTime(weekTime) }}</span>
        </div>
      </footer>
    </template>

    <!-- Stats View -->
    <template v-else>
      <StatsView @back="currentView = 'tasks'" />
    </template>

    <AddTaskModal
      v-model:show="showModal"
      @close="showModal = false"
      @add="handleAddTask"
      @add-start="handleAddTaskStart"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import TodayFocus from "./components/TodayFocus.vue";
import TaskItem from "./components/TaskItem.vue";
import StatsView from "./components/StatsView.vue";
import AddTaskModal from "./components/AddTaskModal.vue";

type View = "tasks" | "stats";
const currentView = ref<View>("tasks");

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
  tasks.value.reduce((sum, t) => sum + t.elapsed + (t.isTracking ? t.sessionTime : 0), 0),
);
const weekTime = ref(28800);

const tagColor = (category: string) => {
  const colors: Record<string, { color: string; textColor: string }> = {
    Work: { color: "#dbeafe", textColor: "#1d4ed8" },
    Design: { color: "#f3f4f6", textColor: "#374151" },
    "Code Review": { color: "#f3f4f6", textColor: "#374151" },
    Meeting: { color: "#fef3c7", textColor: "#b45309" },
  };
  return colors[category] || { color: "#f3f4f6", textColor: "#374151" };
};

const formatTime = (seconds: number) => {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return `${h.toString().padStart(2, "0")}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
};

const toggleTimer = (task: Task) => {
  if (task.isTracking) {
    task.elapsed += task.sessionTime;
    task.sessionTime = 0;
    task.isTracking = false;
  } else {
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

// Timer
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
onMounted(() => startTimer());
onUnmounted(() => stopTimer());
</script>

<style scoped>
.app {
  max-width: 640px;
  margin: 0 auto;
  padding: 32px 24px;
  min-height: 100vh;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.logo {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: -0.5;
}

.nav {
  display: flex;
  align-items: center;
  gap: 16px;
}

.nav-item {
  background: none;
  border: none;
  font-size: 14px;
  color: var(--text-muted);
  cursor: pointer;
  padding: 6px 0;
  transition: color 0.15s;
}

.nav-item:hover {
  color: var(--text-secondary);
}

.nav-item.active {
  color: var(--text-primary);
  font-weight: 500;
}

.icon-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 6px;
  margin-left: 8px;
  transition: color 0.15s;
}

.icon-btn:hover {
  color: var(--text-secondary);
}

.add-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: var(--text-primary);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  padding: 7px 14px;
  cursor: pointer;
  transition: opacity 0.15s;
}

.add-btn:hover {
  opacity: 0.9;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 12px;
}

.count {
  background: var(--border);
  color: var(--text-secondary);
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
}

.task-list {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  overflow: hidden;
}

.main {
  margin-bottom: 24px;
}

.footer {
  display: flex;
  justify-content: space-between;
  padding: 16px 20px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
}

.stat {
  display: flex;
  align-items: center;
  gap: 8px;
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.stat-value {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}
</style>
