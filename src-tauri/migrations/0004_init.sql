-- =========================
-- 分类表：用于任务标签、统计聚合、颜色映射
-- =========================
CREATE TABLE IF NOT EXISTS categories (
  id INTEGER PRIMARY KEY AUTOINCREMENT,          -- 分类ID，自增主键
  name TEXT NOT NULL UNIQUE,                     -- 分类名称（唯一），例如 Work/Study/Life
  color TEXT NOT NULL DEFAULT '#4F8DF7',         -- 分类颜色（Hex），用于 UI Tag 和统计图颜色
  created_at TEXT NOT NULL DEFAULT (datetime('now')) -- 创建时间（UTC，SQLite datetime('now')）
);


-- =========================
-- 任务表：任务本体信息
-- =========================
CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,          -- 任务ID，自增主键
  title TEXT NOT NULL,                           -- 任务标题（必填）
  done INTEGER NOT NULL DEFAULT 0,               -- 是否完成：0=未完成，1-暂停中  2=已完成

  category_id INTEGER,                           -- 分类ID（可选），关联 categories.id
  estimate_seconds INTEGER,                      -- 预估用时（秒，可选），用于“计划 vs 实际”与提示
  notes TEXT,                                    -- 任务备注（可选），任务详情说明/链接/步骤等
  is_today_focus INTEGER NOT NULL DEFAULT 0,      -- 是否加入 Today Focus：0=否，1=是（用于“今天要做什么”区域）

  created_at TEXT NOT NULL DEFAULT (datetime('now')), -- 创建时间（UTC）
  updated_at TEXT NOT NULL DEFAULT (datetime('now')), -- 更新时间（UTC），任务内容变更/状态变更时应更新
  completed_at TEXT,                             -- 完成时间（UTC，可选）：done=1 时写入，便于 Completed 页显示“多久前完成”

  FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL

);

-- 索引：按完成状态过滤/排序（All/Completed 列表常用）
CREATE INDEX IF NOT EXISTS idx_tasks_done ON tasks(done);

-- 索引：按分类筛选（分类过滤、统计聚合常用）
CREATE INDEX IF NOT EXISTS idx_tasks_category_id ON tasks(category_id);

-- 索引：Today Focus 列表（按“今天关注”快速取出）
CREATE INDEX IF NOT EXISTS idx_tasks_today_focus ON tasks(is_today_focus);


-- =========================
-- 计时记录表：每次开始-停止计时生成一条记录（session）
-- =========================
CREATE TABLE IF NOT EXISTS time_entries (
  id INTEGER PRIMARY KEY AUTOINCREMENT,          -- 计时记录ID，自增主键
  task_id INTEGER NOT NULL,                      -- 关联的任务ID（必填），对应 tasks.id
  started_at TEXT NOT NULL,                      -- 计时开始时间（UTC）
  ended_at TEXT,                                 -- 计时结束时间（UTC，可空：空表示正在计时中）
  duration_seconds INTEGER NOT NULL DEFAULT 0,   -- 本条记录的时长（秒）
  note TEXT,                                     -- 本次计时备注（可选），例如“番茄钟 #1 / 会议讨论要点”

  FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
  -- 外键：计时记录属于某个任务
  -- ON DELETE CASCADE：删除任务时自动删除该任务的所有计时记录
);

-- 索引：按任务查询计时记录（任务详情页、统计汇总常用）
CREATE INDEX IF NOT EXISTS idx_time_entries_task_id ON time_entries(task_id);

-- 索引：按开始时间范围查询（Today/Week/Month 统计常用）
CREATE INDEX IF NOT EXISTS idx_time_entries_started_at ON time_entries(started_at);

-- 索引：按结束时间范围查询（统计、清理未结束记录等）
CREATE INDEX IF NOT EXISTS idx_time_entries_ended_at ON time_entries(ended_at);

-- 可选约束：防止同一个任务存在多条“未结束”的计时记录
-- 解释：ended_at IS NULL 表示还在计时中；同一 task_id 最多允许一条进行中的记录
CREATE UNIQUE INDEX IF NOT EXISTS uniq_open_entry_per_task
ON time_entries(task_id)
WHERE ended_at IS NULL;


INSERT OR IGNORE INTO categories (name, color) VALUES
  ('Work', '#3B82F6'),
  ('Study', '#A855F7'),
  ('Life', '#22C55E'),
  ('Health', '#06B6D4'),
  ('Meeting', '#F97316'),
  ('Other', '#9CA3AF');
