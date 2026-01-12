<template>
  <div class="today-focus">
    <div class="focus-label">TODAY FOCUS</div>
    <div class="focus-input">
      <NInput
        :value="inputValue"
        placeholder="Write down what to do today..."
        clearable
        size="medium"
        @keyup.enter="handleSubmit"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { NInput } from "naive-ui";
import { ref } from "vue";

const inputValue = ref("");

const handleSubmit = () => {
  if (inputValue.value.trim()) {
    console.log("提交任务:", inputValue.value);
    inputValue.value = "";
  }
};
</script>

<style scoped>
.today-focus {
  padding: 8px 0;
  margin-bottom: 8px;
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

.focus-input :deep(.n-input) {
  --n-border: none;
  --n-border-radius: 8px;
}
</style>
