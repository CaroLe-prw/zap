<template>
  <div class="today-focus">
    <div class="focus-label">TODAY FOCUS</div>
    <div class="focus-input-wrapper">
      <NInput
        :value="inputValue"
        placeholder="Write down what to do today..."
        clearable
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
}

.focus-label {
  font-size: 12px;
  font-weight: 700;
  color: #111827;
  margin-bottom: 10px;
  letter-spacing: 1.5px;
}

.focus-input-wrapper {
  background: white;
  border-radius: 10px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}
</style>
