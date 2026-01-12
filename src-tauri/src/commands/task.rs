use serde::{Deserialize, Serialize};

struct Task {
    id: u32,
    title: String,
    done: bool,
    category_id: Option<u32>,
    estimate_seconds: Option<u32>,
    notes: Option<String>,
    is_today_focus: bool,
    created_at: String,
    updated_at: String,
    completed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    /// 文本标题
    title: String,
    /// 分类id
    category_id: Option<u32>,
    /// 预估用时
    estimate_seconds: Option<u32>,
    /// 备注
    notes: Option<String>,
    /// 是否加入Today Focus
    is_today_focus: bool,
    /// 是否立即开始并且开始计时
    start_on_create: Option<bool>,
}
