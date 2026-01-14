<template>
  <NModal
    v-model:show="visible"
    preset="dialog"
    title="Add Task"
    class="add-task-modal"
  >
    <div class="modal-content">
      <!-- Task Name -->
      <div class="form-item">
        <label class="form-label"
          >Task Name <span class="required">*</span></label
        >
        <NInput
          v-model:value="form.taskName"
          placeholder="Enter task name"
          maxlength="200"
          show-count
        />
      </div>

      <!-- Category -->
      <div class="form-item">
        <label class="form-label">Category</label>
        <NSelect
          v-model:value="selectedCategory"
          placeholder="Select category"
          :options="categoryOptions"
          :render-label="renderCategoryOption"
        />
      </div>

      <!-- Estimate -->
      <div class="form-item">
        <label class="form-label">Estimate</label>
        <div class="estimate-chips">
          <button
            v-for="chip in estimateChips"
            :key="chip"
            :class="['estimate-btn', { active: selectedEstimate === chip }]"
            @click="selectEstimateChip(chip)"
          >
            {{ chip }}
          </button>
        </div>
        <div v-if="isCustom" class="custom-estimate-row">
          <input
            v-model.number="customHours"
            type="number"
            class="number-input"
            min="0"
            max="23"
            placeholder="0"
            @wheel.prevent="handleHoursWheel"
            @blur="validateHours"
          />
          <span class="unit">h</span>
          <input
            v-model.number="customMinutes"
            type="number"
            class="number-input"
            min="0"
            max="59"
            placeholder="0"
            @wheel.prevent="handleMinutesWheel"
            @blur="validateMinutes"
          />
          <span class="unit">m</span>
        </div>
      </div>

      <!-- Notes -->
      <div class="form-item">
        <label class="form-label">Notes</label>
        <NInput
          v-model:value="form.notes"
          type="textarea"
          placeholder="Add notes..."
          :rows="3"
          maxlength="500"
        />
      </div>

      <!-- Switches -->
      <div class="switches">
        <div class="switch-item">
          <div class="switch-info">
            <span class="switch-label">Start immediately</span>
            <span class="switch-desc">Start timer after create</span>
          </div>
          <NSwitch v-model:value="form.startImmediately" />
        </div>
        <div class="switch-item">
          <div class="switch-info">
            <span class="switch-label">Add to Today Focus</span>
            <span class="switch-desc">Show in Today Focus</span>
          </div>
          <NSwitch v-model:value="form.addToTodayFocus" />
        </div>
      </div>
    </div>

    <!-- Action Buttons -->
    <template #action>
      <div class="action-buttons">
        <NButton @click="handleCancel">Cancel</NButton>
        <NButton type="primary" @click="handleSubmit">
          {{ form.startImmediately ? "Create & Start" : "Create" }}
        </NButton>
      </div>
    </template>
  </NModal>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, h } from "vue";
import { NModal, NSelect, NSwitch, NButton, NInput } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";

interface Category {
  id: number;
  name: string;
  color: string;
}

interface AddTaskForm {
  taskName: string;
  category: string;
  estimate: string | null;
  notes: string;
  startImmediately: boolean;
  addToTodayFocus: boolean;
}

const props = defineProps<{
  show: boolean;
}>();

const emit = defineEmits<{
  (e: "update:show", value: boolean): void;
  (e: "close"): void;
  (
    e: "add",
    data: {
      form: AddTaskForm;
      categoryId: number | null;
      addToFocus: boolean;
      estimateSeconds: number | null;
    },
  ): void;
  (
    e: "add-start",
    data: {
      form: AddTaskForm;
      categoryId: number | null;
      addToFocus: boolean;
      estimateSeconds: number | null;
    },
  ): void;
}>();

const visible = computed({
  get: () => props.show,
  set: (val) => emit("update:show", val),
});

const form = ref<AddTaskForm>({
  taskName: "",
  category: "",
  estimate: null,
  notes: "",
  startImmediately: false,
  addToTodayFocus: false,
});

const selectedCategory = ref<number | null>(null);
const selectedEstimate = ref<string>("");
const customHours = ref(0);
const customMinutes = ref(0);
const isCustom = computed(() => selectedEstimate.value === "Custom");

const categoryOptions = ref<
  Array<{
    label: string;
    value: number;
    color: { color: string; textColor: string };
  }>
>([]);

const loadCategories = async () => {
  try {
    const categories: Category[] = await invoke("list_categories");
    categoryOptions.value = categories.map((cat) => ({
      label: cat.name,
      value: cat.id,
      color: { color: cat.color, textColor: "#fff" },
    }));
  } catch (error) {
    console.error("Failed to load categories:", error);
  }
};

onMounted(() => {
  loadCategories();
});

const renderCategoryOption = (option: {
  label: string;
  value: number;
  color: { color: string };
}) => {
  return h(
    "div",
    { style: { display: "flex", alignItems: "center", gap: "8px" } },
    [
      h("span", {
        style: {
          display: "inline-block",
          width: "8px",
          height: "8px",
          borderRadius: "50%",
          backgroundColor: option.color.color,
        },
      }),
      h("span", option.label),
    ],
  );
};

const estimateChips = ["15m", "30m", "45m", "1h", "Custom"];

const selectEstimateChip = (chip: string) => {
  selectedEstimate.value = chip;
};

const handleHoursWheel = (e: WheelEvent) => {
  const delta = e.deltaY > 0 ? -1 : 1;
  customHours.value = Math.min(23, Math.max(0, customHours.value + delta));
};

const handleMinutesWheel = (e: WheelEvent) => {
  const delta = e.deltaY > 0 ? -5 : 5;
  customMinutes.value = Math.min(59, Math.max(0, customMinutes.value + delta));
};

const validateHours = () => {
  if (customHours.value > 23) customHours.value = 23;
  if (customHours.value < 0 || isNaN(customHours.value)) customHours.value = 0;
};

const validateMinutes = () => {
  if (customMinutes.value > 59) customMinutes.value = 59;
  if (customMinutes.value < 0 || isNaN(customMinutes.value))
    customMinutes.value = 0;
};

// Reset form when modal opens
watch(
  () => props.show,
  (newVal) => {
    if (newVal) {
      form.value = {
        taskName: "",
        category: "",
        estimate: "",
        notes: "",
        startImmediately: false,
        addToTodayFocus: false,
      };
      selectedCategory.value = null;
      selectedEstimate.value = "";
    }
  },
);

const handleCancel = () => {
  emit("close");
  visible.value = false;
};

const estimateToSeconds = (estimate: string): number | null => {
  if (!estimate) return null;
  const match = estimate.match(/(\d+)h(?:(\d+)m)?|(\d+)m/);
  if (!match) return null;
  const h = parseInt(match[1] || "0", 10);
  const m = parseInt(match[2] || match[3] || "0", 10);
  return h * 3600 + m * 60;
};

const handleSubmit = () => {
  if (!form.value.taskName.trim()) return;
  // Find category name from selected ID
  const selectedCat = categoryOptions.value.find(
    (c) => c.value === selectedCategory.value,
  );
  form.value.category = selectedCat?.label || "Other";
  // Handle estimate
  if (isCustom.value) {
    const h = customHours.value || 0;
    const m = customMinutes.value || 0;
    if (h > 0 && m > 0) {
      form.value.estimate = `${h}h${m}m`;
    } else if (h > 0) {
      form.value.estimate = `${h}h`;
    } else {
      form.value.estimate = `${m}m`;
    }
  } else {
    form.value.estimate = selectedEstimate.value;
  }
  const estimateSeconds = estimateToSeconds(form.value.estimate || "");
  if (form.value.startImmediately) {
    emit("add-start", {
      form: { ...form.value },
      categoryId: selectedCategory.value,
      addToFocus: form.value.addToTodayFocus,
      estimateSeconds,
    });
  } else {
    emit("add", {
      form: { ...form.value },
      categoryId: selectedCategory.value,
      addToFocus: form.value.addToTodayFocus,
      estimateSeconds,
    });
  }
  visible.value = false;
};
</script>

<style scoped>
.add-task-modal {
  width: 480px;
}

.modal-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 8px 0;
}

.form-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-item.half {
  flex: 1;
}

.form-row {
  display: flex;
  gap: 16px;
}

.category-input-wrapper {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  background: white;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  flex-wrap: nowrap;
  overflow-x: auto;
}

.category-input-wrapper:focus-within {
  border-color: #3b82f6;
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.category-tags-inline {
  display: flex;
  flex-wrap: nowrap;
  gap: 4px;
  flex-shrink: 0;
}

.category-real-input {
  flex: 1;
  min-width: 320px;
  border: none;
  outline: none;
  background: transparent;
  font-size: 14px;
  color: #374151;
}

.category-real-input::placeholder {
  color: #9ca3af;
}

.estimate-chips {
  display: flex;
  gap: 6px;
  margin-top: 8px;
  flex-wrap: wrap;
}

.estimate-btn {
  padding: 4px 12px;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  background: #f9fafb;
  color: #374151;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.estimate-btn:hover {
  background: #f3f4f6;
}

.estimate-btn.active {
  background: #111827;
  color: #fff;
  border-color: #111827;
}

.custom-estimate-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.number-input {
  width: 60px;
  height: 32px;
  padding: 4px 8px;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
  text-align: center;
  font-size: 14px;
  outline: none;
}

.number-input:focus {
  border-color: #3b82f6;
}

/* Hide spinners */
.number-input::-webkit-inner-spin-button,
.number-input::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.unit {
  font-size: 14px;
  color: #6b7280;
}

.form-label {
  font-size: 13px;
  font-weight: 500;
  color: #374151;
}

.required {
  color: #ef4444;
}

.switches {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-top: 8px;
}

.switch-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 10px;
  background: #f9fafb;
  border-radius: 6px;
}

.switch-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.switch-label {
  font-size: 13px;
  font-weight: 500;
  color: #111827;
}

.switch-desc {
  font-size: 11px;
  color: #9ca3af;
}

.action-buttons {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>
