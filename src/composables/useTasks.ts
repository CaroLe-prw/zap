import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface Task {
  id: number;
  title: string;
  category: string;
  completed: boolean;
  elapsed: number;
  sessionTime: number;
  isTracking: boolean;
}

interface AddTaskData {
  form: {
    taskName: string;
    category: string;
    notes?: string;
  };
  categoryId: number | null;
  addToFocus: boolean;
  estimateSeconds: number | null;
}

export function useTasks() {
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

  const stopOtherTimers = () => {
    tasks.value.forEach((t) => {
      if (t.isTracking) {
        t.elapsed += t.sessionTime;
        t.sessionTime = 0;
        t.isTracking = false;
      }
    });
  };

  const addTask = async (data: AddTaskData) => {
    try {
      await invoke("add_task", {
        createTask: {
          title: data.form.taskName,
          category_id: data.categoryId,
          estimate_seconds: data.estimateSeconds,
          notes: data.form.notes || null,
          is_today_focus: data.addToFocus,
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
      completed: false,
      elapsed: 0,
      sessionTime: 0,
      isTracking: false,
    });
  };

  const addTaskAndStart = async (data: AddTaskData) => {
    stopOtherTimers();
    try {
      await invoke("add_task", {
        createTask: {
          title: data.form.taskName,
          category_id: data.categoryId,
          estimate_seconds: data.estimateSeconds,
          notes: data.form.notes || null,
          is_today_focus: data.addToFocus,
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
      completed: false,
      elapsed: 0,
      sessionTime: 0,
      isTracking: true,
    });
  };

  const toggleTimer = (task: Task) => {
    if (task.isTracking) {
      task.elapsed += task.sessionTime;
      task.sessionTime = 0;
      task.isTracking = false;
    } else {
      stopOtherTimers();
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
