import { ref, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface Task {
  id: number;
  title: string;
  category: string;
  color: string | null;
  completed: boolean;
  elapsed: number;
  sessionTime: number;
  isTracking: boolean;
  isFinishing: boolean;
  totalDurationSeconds: number;
  todayDuration: number;
  completedAt: string | null;
}

interface AddTaskData {
  form: {
    taskName: string;
    category: string;
    notes?: string;
  };
  categoryId: number | null;
  estimateSeconds: number | null;
}

export function useTasks() {
  const tasks = ref<Task[]>([]);

  const stopOtherTimers = async () => {
    for (const t of tasks.value) {
      if (t.isTracking) {
        try {
          await invoke("stop_task", { taskId: t.id });
        } catch (error) {
          console.error("Failed to stop task:", error);
        }
        t.elapsed += t.sessionTime;
        t.sessionTime = 0;
        t.isTracking = false;
      }
    }
  };

  const addTask = async (data: AddTaskData) => {
    try {
      await invoke("add_task", {
        createTask: {
          title: data.form.taskName,
          category_id: data.categoryId,
          estimate_seconds: data.estimateSeconds,
          notes: data.form.notes || null,
          start_on_create: false,
        },
      });
    } catch (error) {
      console.error("Failed to add task:", error);
    }
    const category = data.form.category || "Other";
    tasks.value.unshift({
      id: Date.now(),
      title: data.form.taskName,
      category,
      color: null,
      completed: false,
      elapsed: 0,
      sessionTime: 0,
      isTracking: false,
      isFinishing: false,
      totalDurationSeconds: 0,
      todayDuration: 0,
      completedAt: null,
    });
  };

  const addTaskAndStart = async (data: AddTaskData) => {
    await stopOtherTimers();
    try {
      await invoke("add_task", {
        createTask: {
          title: data.form.taskName,
          category_id: data.categoryId,
          estimate_seconds: data.estimateSeconds,
          notes: data.form.notes || null,
          start_on_create: true,
        },
      });
    } catch (error) {
      console.error("Failed to add task:", error);
    }
    const category = data.form.category || "Other";
    tasks.value.unshift({
      id: Date.now(),
      title: data.form.taskName,
      category,
      color: null,
      completed: false,
      elapsed: 0,
      sessionTime: 0,
      isTracking: true,
      isFinishing: false,
      totalDurationSeconds: 0,
      todayDuration: 0,
      completedAt: null,
    });
  };

  const toggleTimer = async (task: Task) => {
    if (task.isTracking) {
      try {
        await invoke("stop_task", { taskId: task.id });
      } catch (error) {
        console.error("Failed to stop task:", error);
      }
      task.elapsed += task.sessionTime;
      task.totalDurationSeconds += task.sessionTime;
      task.todayDuration += task.sessionTime;
      task.sessionTime = 0;
      task.isTracking = false;
    } else {
      await stopOtherTimers();
      try {
        await invoke("start_task", { taskId: task.id });
      } catch (error) {
        console.error("Failed to start task:", error);
      }
      task.sessionTime = 0;
      task.isTracking = true;
    }
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

  return {
    tasks,
    addTask,
    addTaskAndStart,
    toggleTimer,
  };
}
