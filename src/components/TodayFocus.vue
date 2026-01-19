<template>
  <div class="today-focus">
    <div class="focus-label">TODAY FOCUS</div>

    <!-- Empty State: Input -->
    <div v-if="!focus" class="focus-input">
      <input
        v-model="inputValue"
        type="text"
        placeholder="Write down the ONE thing you must do today..."
        class="focus-text-input"
        @keyup.enter="handleSubmit"
      />
    </div>

    <!-- Filled State: Card -->
    <div
      v-else
      class="focus-card"
      :class="{ 'is-done': focus.is_done }"
      @click="handleToggle"
    >
      <div class="card-check">
        <div class="check-circle" :class="{ checked: focus.is_done }">
          <svg
            v-if="focus.is_done"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="3"
          >
            <polyline points="20 6 9 17 4 12" />
          </svg>
        </div>
      </div>
      <div class="card-content">
        <span class="card-text" :class="{ 'text-done': focus.is_done }">
          {{ focus.content }}
        </span>
        <span v-if="focus.is_done" class="card-badge">Completed!</span>
      </div>
      <button class="edit-btn" @click.stop="handleEdit" title="Edit">
        <svg
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
        </svg>
      </button>
    </div>

    <!-- Confetti Overlay -->
    <div v-if="showConfetti" class="confetti-container">
      <div v-for="i in 50" :key="i" class="confetti" :style="confettiStyle(i)" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface DailyFocus {
  id: number;
  focus_date: string;
  content: string;
  is_done: number;
  position: number;
  created_at: string;
  updated_at: string;
}

const focus = ref<DailyFocus | null>(null);
const inputValue = ref("");
const showConfetti = ref(false);

const getTodayDate = () => {
  const now = new Date();
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}-${String(now.getDate()).padStart(2, "0")}`;
};

const loadFocus = async () => {
  try {
    const result = await invoke<DailyFocus | null>("get_daily_focus", {
      date: getTodayDate(),
    });
    focus.value = result;
  } catch (error) {
    console.error("Failed to load daily focus:", error);
  }
};

const handleSubmit = async () => {
  const content = inputValue.value.trim();
  if (!content) return;

  try {
    const result = await invoke<DailyFocus>("upsert_daily_focus", {
      req: { date: getTodayDate(), content },
    });
    focus.value = result;
    inputValue.value = "";
  } catch (error) {
    console.error("Failed to save daily focus:", error);
  }
};

const handleToggle = async () => {
  if (!focus.value) return;

  try {
    const result = await invoke<DailyFocus>("toggle_daily_focus", {
      id: focus.value.id,
    });
    const wasNotDone = focus.value.is_done === 0;
    focus.value = result;

    if (wasNotDone && result.is_done === 1) {
      triggerConfetti();
    }
  } catch (error) {
    console.error("Failed to toggle daily focus:", error);
  }
};

const handleEdit = () => {
  if (focus.value) {
    inputValue.value = focus.value.content;
    focus.value = null;
  }
};

const triggerConfetti = () => {
  showConfetti.value = true;
  setTimeout(() => {
    showConfetti.value = false;
  }, 3000);
};

const confettiColors = [
  "#ff6b6b",
  "#ffd93d",
  "#6bcb77",
  "#4d96ff",
  "#ff9f43",
  "#a55eea",
  "#26de81",
  "#fd79a8",
];

const confettiStyle = (i: number) => {
  const color = confettiColors[i % confettiColors.length];
  const left = Math.random() * 100;
  const delay = Math.random() * 0.5;
  const duration = 2 + Math.random() * 2;
  const size = 8 + Math.random() * 8;

  return {
    "--color": color,
    "--left": `${left}%`,
    "--delay": `${delay}s`,
    "--duration": `${duration}s`,
    "--size": `${size}px`,
  };
};

onMounted(() => {
  loadFocus();
});
</script>

<style scoped>
.today-focus {
  padding: 8px 0;
  margin-bottom: 8px;
  position: relative;
}

.focus-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  margin-bottom: 10px;
  letter-spacing: 1.5px;
}

.focus-input {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
}

.focus-text-input {
  width: 100%;
  padding: 14px 16px;
  font-size: 15px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: var(--text-primary);
  outline: none;
  box-sizing: border-box;
}

.focus-text-input::placeholder {
  color: var(--text-muted);
  font-style: italic;
}

.focus-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 18px 20px;
  background: #fffbeb;
  border: 1px solid #fde68a;
  border-left: 4px solid #f59e0b;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.focus-card:hover {
  background: #fef3c7;
  border-left-color: #d97706;
}

.focus-card.is-done {
  background: #ecfdf5;
  border-color: #d1fae5;
  border-left-color: #10b981;
}

.focus-card.is-done:hover {
  background: #d1fae5;
  border-left-color: #059669;
}

.card-check {
  flex-shrink: 0;
}

.check-circle {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid #fbbf24;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  color: transparent;
}

.check-circle.checked {
  background: #10b981;
  border-color: #10b981;
  color: white;
}

.card-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.card-text {
  font-size: 16px;
  font-weight: 500;
  color: #78350f;
  line-height: 1.4;
  word-break: break-word;
}

.card-text.text-done {
  text-decoration: line-through;
  color: #064e3b;
}

.card-badge {
  font-size: 11px;
  font-weight: 600;
  color: #064e3b;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.edit-btn {
  flex-shrink: 0;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: #f59e0b;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  opacity: 0;
}

.focus-card:hover .edit-btn {
  opacity: 1;
}

.edit-btn:hover {
  background: #fde68a;
  color: #b45309;
}

.focus-card.is-done .edit-btn {
  color: #6ee7b7;
}

.focus-card.is-done .edit-btn:hover {
  background: #d1fae5;
  color: #059669;
}

/* Confetti Animation */
.confetti-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 9999;
  overflow: hidden;
}

.confetti {
  position: absolute;
  width: var(--size);
  height: var(--size);
  background: var(--color);
  left: var(--left);
  top: -20px;
  opacity: 1;
  animation: confetti-fall var(--duration) ease-out var(--delay) forwards;
}

.confetti:nth-child(odd) {
  border-radius: 50%;
}

.confetti:nth-child(even) {
  border-radius: 2px;
  transform: rotate(45deg);
}

@keyframes confetti-fall {
  0% {
    top: -20px;
    opacity: 1;
    transform: translateX(0) rotate(0deg);
  }
  100% {
    top: 100vh;
    opacity: 0;
    transform: translateX(calc((var(--left) - 50%) * 0.5)) rotate(720deg);
  }
}
</style>
