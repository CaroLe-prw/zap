use serde::Serialize;

/// 分页响应结构
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    /// 当前页的数据列表
    pub data: Vec<T>,
    /// 符合条件的总记录数
    pub total: u32,
    /// 当前页码（从 1 开始）
    pub page_index: u32,
    /// 每页大小
    pub page_size: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: u32, page_index: u32, page_size: u32) -> Self {
        Self {
            data,
            total,
            page_index,
            page_size,
        }
    }

    pub fn empty(page_index: u32, page_size: u32) -> Self {
        Self {
            data: Vec::new(),
            total: 0,
            page_index,
            page_size,
        }
    }
}
