<template>
  <div class="app">
    <!-- Task View -->
    <template v-if="currentView === 'tasks'">
      <header class="header">
        <nav class="tabs">
          <button
            v-for="tab in tabs"
            :key="tab"
            class="tab-item"
            :class="{ active: currentTab === tab }"
            @click="currentTab = tab"
          >
            {{ tab }}
          </button>
        </nav>
        <nav class="nav">
          <button
            class="icon-btn"
            title="Statistics"
            @click="currentView = 'stats'"
          >
            <svg
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <path d="M18 20V10M12 20V4M6 20v-6" />
            </svg>
          </button>
          <button class="add-btn" @click="showModal = true">
            <svg
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2.5"
            >
              <line x1="12" y1="5" x2="12" y2="19" />
              <line x1="5" y1="12" x2="19" y2="12" />
            </svg>
            Add Task
          </button>
        </nav>
      </header>

      <TodayFocus />

      <main class="main">
        <div class="section-header">
          <div class="section-title">
            <span>{{ currentTab }}</span>
            <span class="count">{{ filteredTasks.length }}</span>
          </div>
          <div class="search-box">
            <svg
              class="search-icon"
              width="14"
              height="14"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
            >
              <circle cx="11" cy="11" r="8" />
              <line x1="21" y1="21" x2="16.65" y2="16.65" />
            </svg>
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search..."
              class="search-input"
              @input="handleSearch"
            />
          </div>
        </div>
        <div class="task-list">
          <TaskItem
            v-for="task in filteredTasks"
            :key="`${currentTab}-${currentPage}-${task.id}`"
            :task="task"
            :format-time="formatTime"
            :format-time-short="formatTimeShort"
            @toggle="toggleTimer"
            @complete="handleComplete"
          />
          <div v-if="filteredTasks.length === 0" class="empty-state">
            <p>
              No
              {{ currentTab === "All Tasks" ? "" : currentTab.toLowerCase() }}
              tasks yet
            </p>
          </div>
        </div>
      </main>

      <footer class="footer">
        <div class="stat">
          <span class="stat-label">Total</span>
          <span class="stat-value">{{ formatTimeShort(totalTime) }}</span>
        </div>
        <div v-if="totalPages > 1" class="pagination">
          <button
            class="page-btn"
            :disabled="currentPage === 1"
            @click="prevPage"
          >
            <
          </button>
          <span class="page-info">{{ currentPage }} / {{ totalPages }}</span>
          <button
            class="page-btn"
            :disabled="currentPage === totalPages"
            @click="nextPage"
          >
            >
          </button>
        </div>
        <div class="stat">
          <span class="stat-label">This week</span>
          <span class="stat-value">{{ formatTimeShort(weekTime) }}</span>
        </div>
      </footer>

      <!-- Undo Toast -->
      <div v-if="undoToast.visible" class="undo-toast">
        <span>Task Completed.</span>
        <button class="undo-btn" @click="handleUndo">Undo</button>
      </div>
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
import { ref, computed, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import TodayFocus from "./components/TodayFocus.vue";
import TaskItem from "./components/TaskItem.vue";
import StatsView from "./components/StatsView.vue";
import AddTaskModal from "./components/AddTaskModal.vue";
import { useTasks, type Task } from "./composables/useTasks";

const { tasks, addTask, addTaskAndStart, toggleTimer } = useTasks();

// Undo Toast 状态
const undoToast = ref({ visible: false, taskId: 0, taskTitle: "" });

type View = "tasks" | "stats";
const currentView = ref<View>("tasks");

const tabs = ["All Tasks", "In Progress", "Completed"];
const currentTab = ref("All Tasks");
const showModal = ref(false);
const searchQuery = ref("");
const currentPage = ref(1);
const totalPages = ref(1);

const loadTasks = async (page = 1) => {
  try {
    const result = await invoke("list_tasks", {
      req: {
        page_index: page,
        page_size: 5,
        task_name: searchQuery.value || null,
        done:
          currentTab.value === "All Tasks"
            ? null
            : currentTab.value === "In Progress"
              ? 1
              : 2,
      },
    });

    const response = result as any;
    if (response.data) {
      currentPage.value = response.page_index;
      totalPages.value = Math.ceil(response.total / response.page_size);
      tasks.value = response.data.map((t: any) => ({
        id: t.task_id,
        title: t.title,
        category: t.category_name || "Other",
        color: t.color,
        completed: t.done === 2,
        elapsed: 0,
        sessionTime: t.session_seconds || 0,
        isTracking: t.done === 1,
        isFinishing: false,
        totalDurationSeconds: t.total_duration_seconds || 0,
        todayDuration: t.today_duration_seconds || 0,
        completedAt: t.completed_at,
      }));
    }
  } catch (error) {
    console.error("Failed to load tasks:", error);
  }
};

const handleSearch = () => {
  currentPage.value = 1;
  loadTasks();
};

const prevPage = () => {
  if (currentPage.value > 1) {
    loadTasks(currentPage.value - 1);
  }
};

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    loadTasks(currentPage.value + 1);
  }
};

watch(currentTab, () => {
  currentPage.value = 1;
  loadTasks();
});

onMounted(() => {
  loadTasks();
});

const filteredTasks = computed(() => {
  if (currentTab.value === "All Tasks") return tasks.value;
  if (currentTab.value === "In Progress")
    return tasks.value.filter((t) => t.isTracking);
  return tasks.value.filter((t) => t.completed);
});

const totalTime = computed(() =>
  tasks.value.reduce(
    (sum, t) => sum + t.elapsed + (t.isTracking ? t.sessionTime : 0),
    0,
  ),
);
const weekTime = ref(28800);

const formatTime = (seconds: number) => {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return `${h.toString().padStart(2, "0")}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
};

const formatTimeShort = (seconds: number) => {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  if (seconds === 0) {
    return "0h 0m";
  }
  if (h > 0) {
    return `${h}h ${m}m`;
  }
  return `${m}m`;
};

const finishTask = async (taskId: number) => {
  try {
    await invoke("finish_task", { taskId });
  } catch (error) {
    console.error("Failed to finish task:", error);
  }
};

const toggleTaskDone = async (taskId: number) => {
  try {
    await invoke("toggle_task_done", { taskId });
  } catch (error) {
    console.error("Failed to toggle task done:", error);
  }
};

const handleComplete = async (task: Task) => {
  if (task.completed) {
    // 在 Completed 列表，点击勾选取消完成
    await toggleTaskDone(task.id);
    task.completed = false;
    await loadTasks();
  } else {
    // 未完成状态，点击勾选完成任务
    // 阶段一：任务变灰打勾
    task.isFinishing = true;
    await finishTask(task.id);
    task.completed = true;
    task.isFinishing = false;

    // 阶段二：延迟后移除任务并显示 Undo Toast
    setTimeout(() => {
      const index = tasks.value.findIndex((t) => t.id === task.id);
      if (index > -1) {
        tasks.value.splice(index, 1);
      }
      undoToast.value = {
        visible: true,
        taskId: task.id,
        taskTitle: task.title,
      };

      // 5秒后自动隐藏
      setTimeout(() => {
        undoToast.value.visible = false;
      }, 5000);
    }, 800);
  }
};

const handleUndo = async () => {
  await toggleTaskDone(undoToast.value.taskId);
  undoToast.value.visible = false;
  // 刷新列表让任务恢复
  await loadTasks();
};

const handleAddTask = async (data: {
  form: any;
  categoryId: number | null;
  addToFocus: boolean;
  estimateSeconds: number | null;
}) => {
  await addTask(data);
  await loadTasks();
};

const handleAddTaskStart = async (data: {
  form: any;
  categoryId: number | null;
  addToFocus: boolean;
  estimateSeconds: number | null;
}) => {
  await addTaskAndStart(data);
  await loadTasks();
  currentTab.value = "In Progress";
};
</script>

<style scoped>
.app {
  max-width: 640px;
  margin: 0 auto;
  padding: 32px 24px 120px;
  min-height: 100vh;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.tabs {
  display: flex;
  align-items: center;
  gap: 24px;
}

.tab-item {
  background: none;
  border: none;
  font-size: 14px;
  color: var(--text-muted);
  cursor: pointer;
  padding: 6px 0;
  transition: color 0.15s;
}

.tab-item:hover {
  color: var(--text-secondary);
}

.tab-item.active {
  color: var(--text-primary);
  font-weight: 500;
}

.nav {
  display: flex;
  align-items: center;
  gap: 16px;
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
  justify-content: space-between;
  margin-bottom: 12px;
}

.section-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 1px;
}

.search-box {
  position: relative;
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
}

.search-input {
  width: 100%;
  padding: 6px 12px 6px 36px;
  font-size: 13px;
  height: 32px;
  box-sizing: border-box;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  background: var(--bg-card);
  color: var(--text-primary);
  outline: none;
  transition: border-color 0.15s;
}

.search-input::placeholder {
  color: var(--text-muted);
}

.search-input:focus {
  border-color: var(--text-primary);
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

.empty-state {
  padding: 48px 20px;
  text-align: center;
  color: var(--text-muted);
  font-size: 14px;
}

.footer {
  position: fixed;
  left: 50%;
  transform: translateX(-50%);
  bottom: 24px;
  width: calc(100% - 48px);
  max-width: 592px;
  display: flex;
  justify-content: space-between;
  padding: 16px 20px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  box-sizing: border-box;
}

.stat {
  display: flex;
  align-items: center;
  gap: 10px;
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

.pagination {
  display: flex;
  align-items: center;
  gap: 8px;
}

.page-btn {
  background: none;
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 4px 10px;
  font-size: 13px;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.15s;
}

.page-btn:hover:not(:disabled) {
  background: var(--bg-page);
}

.page-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.page-info {
  font-size: 13px;
  color: var(--text-secondary);
  font-variant-numeric: tabular-nums;
}

.undo-toast {
  position: fixed;
  left: 50%;
  transform: translateX(-50%);
  bottom: 100px;
  background: #1a1a1a;
  color: #fff;
  padding: 10px 16px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 13px;
  z-index: 100;
  animation: slideUp 0.3s ease;
}

.undo-btn {
  background: none;
  border: none;
  color: #fff;
  font-weight: 600;
  cursor: pointer;
  padding: 0;
  font-size: 13px;
}

.undo-btn:hover {
  text-decoration: underline;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }
}
</style>
