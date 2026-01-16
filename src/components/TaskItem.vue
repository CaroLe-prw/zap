<template>
  <div class="task-item" :class="{ active: task.isTracking }">
    <div class="task-content">
      <div class="task-main">
        <NCheckbox
          :checked="task.completed"
          @update:checked="$emit('complete', task, $event)"
        />
        <span class="task-title" :class="{ completed: task.completed }">
          {{ task.title }}
        </span>
        <NTag
          v-if="task.category"
          size="small"
          :bordered="false"
          :color="tagColor(task.category)"
        >
          {{ task.category }}
        </NTag>
      </div>
      <div
        v-if="task.isTracking || task.totalDurationSeconds > 0"
        class="task-tracking"
      >
        <span v-if="task.isTracking" class="tracking-dot"></span>
        <span class="tracking-time">{{
          formatTime(
            task.totalDurationSeconds +
              (task.isTracking ? task.sessionTime : 0),
          )
        }}</span>
      </div>
    </div>
    <button
      class="timer-btn"
      :class="{ tracking: task.isTracking }"
      @click="$emit('toggle', task)"
    >
      {{ task.isTracking ? "Pause" : "Start" }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { NCheckbox, NTag } from "naive-ui";

interface Task {
  id: number;
  title: string;
  category: string;
  completed: boolean;
  elapsed: number;
  sessionTime: number;
  isTracking: boolean;
  totalDurationSeconds: number;
}

defineProps<{
  task: Task;
  tagColor: (category: string) => { color: string; textColor: string };
  formatTime: (seconds: number) => string;
}>();

defineEmits<{
  (e: "toggle", task: Task): void;
  (e: "complete", task: Task, completed: boolean): void;
}>();
</script>

<style scoped>
.task-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 16px;
  border-bottom: 1px solid var(--border);
  transition: background 0.15s;
}

.task-item:last-child {
  border-bottom: none;
}

.task-item.active {
  background: #f0f9ff;
}

.task-content {
  flex: 1;
  min-width: 0;
}

.task-main {
  display: flex;
  align-items: center;
  gap: 10px;
}

.task-title {
  font-size: 14px;
  color: var(--text-primary);
}

.task-title.completed {
  text-decoration: line-through;
  color: var(--text-muted);
}

.task-tracking {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 4px;
  margin-left: 26px;
}

.tracking-dot {
  width: 6px;
  height: 6px;
  background: #10b981;
  border-radius: 50%;
  animation: pulse 1.5s ease-in-out infinite;
}

.tracking-time {
  font-size: 12px;
  color: #10b981;
  font-variant-numeric: tabular-nums;
  font-weight: 500;
}

.timer-btn {
  padding: 6px 14px;
  font-size: 12px;
  font-weight: 500;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: white;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.15s;
}

.timer-btn:hover {
  background: var(--bg-page);
}

.timer-btn.tracking {
  background: var(--text-primary);
  color: white;
  border-color: var(--text-primary);
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
