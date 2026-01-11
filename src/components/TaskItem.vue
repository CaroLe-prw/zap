<template>
  <div class="task-item" :class="{ active: task.isTracking }">
    <div class="task-content">
      <div class="task-main">
        <NCheckbox v-model:checked="task.completed" />
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
      <div v-if="task.isTracking || task.elapsed > 0" class="task-tracking">
        <span v-if="task.isTracking" class="tracking-dot"></span>
        <span class="tracking-time">{{
          formatTime(task.isTracking ? task.sessionTime : task.elapsed)
        }}</span>
      </div>
    </div>
    <NButton
      :type="task.isTracking ? 'info' : 'success'"
      size="small"
      @click="$emit('toggle', task)"
    >
      <template #icon>
        <NIcon><component :is="task.isTracking ? Pause : Play" /></NIcon>
      </template>
      {{ task.isTracking ? "Pause" : "Start" }}
    </NButton>
  </div>
</template>

<script setup lang="ts">
import { NCheckbox, NTag, NButton, NIcon } from "naive-ui";
import { Play, Pause } from "@vicons/ionicons5";

interface Task {
  id: number;
  title: string;
  category: string;
  completed: boolean;
  elapsed: number;
  sessionTime: number;
  isTracking: boolean;
}

defineProps<{
  task: Task;
  tagColor: (category: string) => { color: string; textColor: string };
  formatTime: (seconds: number) => string;
}>();

defineEmits<{
  (e: "toggle", task: Task): void;
}>();
</script>

<style scoped>
.task-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 14px 0;
  border-bottom: 1px solid #f3f4f6;
}

.task-item.active {
  background: linear-gradient(
    90deg,
    rgba(59, 130, 246, 0.12) 20%,
    transparent 100%
  );
  margin: 4px 0;
  padding: 10px 12px;
  border-radius: 8px;
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
  font-weight: 500;
  color: #111827;
}

.task-title.completed {
  text-decoration: line-through;
  color: #9ca3af;
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
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.5;
    transform: scale(0.8);
  }
}
</style>
