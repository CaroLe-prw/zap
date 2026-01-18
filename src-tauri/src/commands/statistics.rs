use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::error::ZapError;

/// 日期范围查询参数
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StatsDateQuery {
    pub start_date: Option<String>, // YYYY-MM-DD 格式
    pub end_date: Option<String>,   // YYYY-MM-DD 格式
}

/// 分类统计项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStat {
    pub name: String,
    pub color: String,
    pub seconds: i64,
    pub percentage: f64,
}

/// 每日统计项（周视图）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStat {
    pub day_name: String,
    pub date: String,
    pub seconds: i64,
    pub percentage: f64,
}

/// 每月每日统计项（月视图）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyDailyStat {
    pub date: String,
    pub seconds: i64,
    pub active: bool,
    pub level: i32, // 0=empty, 1=low, 2=medium, 3=high
}

/// 任务统计项（详细报告/Top任务）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStat {
    pub task_id: u32,
    pub task_title: String,
    pub category: Option<String>,
    pub category_color: Option<String>,
    pub seconds: i64,
    pub last_time: Option<String>, // 仅 Today 视图使用
}

/// 今日统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodayStatsResponse {
    pub total_seconds: i64,
    pub focused_seconds: i64,
    pub sessions_count: i64,
    pub categories: Vec<CategoryStat>,
    pub detailed_report: Vec<TaskStat>,
}

/// 本周统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekStatsResponse {
    pub total_seconds: i64,
    pub daily_average_seconds: i64,
    pub sessions_count: i64,
    pub daily_breakdown: Vec<DailyStat>,
    pub categories: Vec<CategoryStat>,
}

/// 本月统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthStatsResponse {
    pub total_seconds: i64,
    pub daily_average_seconds: i64,
    pub active_days: i64,
    pub monthly_overview: Vec<MonthlyDailyStat>,
    pub categories: Vec<CategoryStat>,
    pub top_tasks: Vec<TaskStat>,
}

/// 获取今日统计
pub async fn get_today_stats_impl(
    pool: &SqlitePool,
    query: StatsDateQuery,
) -> Result<TodayStatsResponse, ZapError> {
    let date_filter = if let Some(start) = &query.start_date {
        format!("date('{}')", start)
    } else {
        "date('now')".to_string()
    };

    // 总时间
    let total_seconds: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT COALESCE(SUM(duration_seconds), 0) +
               COALESCE((SELECT strftime('%s', 'now') - strftime('%s', started_at)
                        FROM time_entries WHERE ended_at IS NULL AND date(started_at) = {}), 0)
        FROM time_entries
        WHERE date(started_at) = {} AND ended_at IS NOT NULL
    "#,
        date_filter, date_filter
    ))
    .fetch_one(pool)
    .await?;

    // 专注时间
    let focused_seconds: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT COALESCE(SUM(duration_seconds), 0) +
               COALESCE((SELECT strftime('%s', 'now') - strftime('%s', te.started_at)
                        FROM time_entries te
                        JOIN tasks t ON te.task_id = t.id
                        WHERE te.ended_at IS NULL AND t.done = 1 AND date(te.started_at) = {}), 0)
        FROM time_entries te
        JOIN tasks t ON te.task_id = t.id
        WHERE t.done = 1 AND date(te.started_at) = {}
    "#,
        date_filter, date_filter
    ))
    .fetch_one(pool)
    .await?;

    // 会话数
    let sessions_count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM time_entries WHERE date(started_at) = {}",
        date_filter
    ))
    .fetch_one(pool)
    .await?;

    // 分类统计
    let categories: Vec<CategoryStat> = sqlx::query_as::<_, CategoryStatSql>(&format!(
        r#"
        SELECT COALESCE(c.name, 'Other') AS name, COALESCE(c.color, '#9CA3AF') AS color,
               COALESCE(SUM(te.duration_seconds), 0) AS seconds
        FROM time_entries te
        LEFT JOIN tasks t ON te.task_id = t.id
        LEFT JOIN categories c ON t.category_id = c.id
        WHERE date(te.started_at) = {}
        GROUP BY c.id
        ORDER BY seconds DESC
    "#,
        date_filter
    ))
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|c| {
        let total = if total_seconds > 0 {
            total_seconds as f64
        } else {
            1.0
        };
        CategoryStat {
            name: c.name,
            color: c.color,
            seconds: c.seconds,
            percentage: (c.seconds as f64 / total * 100.0).round(),
        }
    })
    .collect();

    // 详细报告
    let detailed_report: Vec<TaskStat> = sqlx::query_as::<_, TaskStatSql>(&format!(
        r#"
        SELECT t.id AS task_id, t.title AS task_title, c.name AS category,
               c.color AS category_color,
               COALESCE(SUM(te.duration_seconds), 0) AS seconds,
               (SELECT MAX(ended_at) FROM time_entries WHERE task_id = t.id AND date(started_at) = {}
                ORDER BY started_at DESC LIMIT 1) AS last_time
        FROM time_entries te
        JOIN tasks t ON te.task_id = t.id
        LEFT JOIN categories c ON t.category_id = c.id
        WHERE date(te.started_at) = {}
        GROUP BY t.id
        ORDER BY seconds DESC, last_time DESC
    "#,
        date_filter, date_filter
    ))
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|t| TaskStat {
        task_id: t.task_id,
        task_title: t.task_title,
        category: t.category,
        category_color: t.category_color,
        seconds: t.seconds,
        last_time: t.last_time.map(|s| format_time_only(&s)),
    })
    .collect();

    Ok(TodayStatsResponse {
        total_seconds,
        focused_seconds,
        sessions_count,
        categories,
        detailed_report,
    })
}

/// 获取本周统计
pub async fn get_week_stats_impl(
    pool: &SqlitePool,
    query: StatsDateQuery,
) -> Result<WeekStatsResponse, ZapError> {
    let start_filter = query
        .start_date
        .as_deref()
        .unwrap_or("date('now', '-6 days')");
    let end_filter = query.end_date.as_deref().unwrap_or("date('now')");

    let date_where = format!(
        "date(started_at) >= date('{}') AND date(started_at) <= date('{}')",
        start_filter, end_filter
    );

    // 总时间
    let total_seconds: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT COALESCE(SUM(duration_seconds), 0) +
               COALESCE((SELECT strftime('%s', 'now') - strftime('%s', started_at)
                        FROM time_entries WHERE ended_at IS NULL AND {}), 0)
        FROM time_entries
        WHERE {} AND ended_at IS NOT NULL
    "#,
        date_where, date_where
    ))
    .fetch_one(pool)
    .await?;

    // 会话数
    let sessions_count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM time_entries WHERE {}",
        date_where
    ))
    .fetch_one(pool)
    .await?;

    // 计算天数
    let days_count: i64 = sqlx::query_scalar(&format!(
        "SELECT CAST((julianday(date('{}')) - julianday(date('{}')) + 1) AS INTEGER)",
        end_filter, start_filter
    ))
    .fetch_one(pool)
    .await?;
    let days_count = if days_count > 0 { days_count } else { 7 };

    // 日均
    let daily_average_seconds = if total_seconds > 0 {
        total_seconds / days_count
    } else {
        0
    };

    // 每日分布
    let daily_breakdown: Vec<DailyStat> = sqlx::query_as::<_, DailyStatSql>(&format!(
        r#"
        SELECT strftime('%w', started_at) AS day_num,
               date(started_at) AS date,
               COALESCE(SUM(duration_seconds), 0) +
               COALESCE((SELECT strftime('%s', 'now') - strftime('%s', started_at)
                        FROM time_entries WHERE ended_at IS NULL AND date(started_at) = date(te.started_at)), 0) AS seconds
        FROM time_entries te
        WHERE {}
        GROUP BY date(started_at)
        ORDER BY date(started_at)
    "#,
        date_where
    ))
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|d| {
        let day_names = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let total = if total_seconds > 0 { total_seconds as f64 } else { 1.0 };
        let day_idx = d.day_num.parse::<usize>().unwrap_or(0) % 7;
        DailyStat {
            day_name: day_names[day_idx].to_string(),
            date: d.date,
            seconds: d.seconds,
            percentage: (d.seconds as f64 / total * 100.0).round(),
        }
    })
    .collect();

    // 分类统计
    let categories: Vec<CategoryStat> = sqlx::query_as::<_, CategoryStatSql>(&format!(
        r#"
        SELECT COALESCE(c.name, 'Other') AS name, COALESCE(c.color, '#9CA3AF') AS color,
               COALESCE(SUM(te.duration_seconds), 0) AS seconds
        FROM time_entries te
        LEFT JOIN tasks t ON te.task_id = t.id
        LEFT JOIN categories c ON t.category_id = c.id
        WHERE {}
        GROUP BY c.id
        ORDER BY seconds DESC
    "#,
        date_where
    ))
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|c| {
        let total = if total_seconds > 0 {
            total_seconds as f64
        } else {
            1.0
        };
        CategoryStat {
            name: c.name,
            color: c.color,
            seconds: c.seconds,
            percentage: (c.seconds as f64 / total * 100.0).round(),
        }
    })
    .collect();

    Ok(WeekStatsResponse {
        total_seconds,
        daily_average_seconds,
        sessions_count,
        daily_breakdown,
        categories,
    })
}

/// 获取本月统计
pub async fn get_month_stats_impl(
    pool: &SqlitePool,
    query: StatsDateQuery,
) -> Result<MonthStatsResponse, ZapError> {
    // 解析前端传来的 YYYY-MM 格式
    let (start_date, end_date) = if let Some(ref start) = query.start_date {
        if start.len() == 7 {
            // YYYY-MM 格式
            let start_sql = format!("date('{}-01')", start);
            // 计算月末：下月第一天减一天
            let end_sql = format!("date('{}-01', '+1 month', '-1 day')", start);
            (start_sql, end_sql)
        } else {
            // YYYY-MM-DD 格式
            let start_sql = format!("date('{}')", start);
            let end_sql = query.end_date.as_ref().map_or_else(
                || start_sql.clone(),
                |e| format!("date('{}')", e)
            );
            (start_sql, end_sql)
        }
    } else {
        // 默认本月
        let start_sql = "date('now', 'start of month')".to_string();
        let end_sql = "date('now')".to_string();
        (start_sql, end_sql)
    };

    let date_where = format!(
        "date(started_at) >= {} AND date(started_at) <= {}",
        start_date, end_date
    );

    // 总时间
    let total_seconds: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT COALESCE(SUM(duration_seconds), 0) +
               COALESCE((SELECT strftime('%s', 'now') - strftime('%s', started_at)
                        FROM time_entries WHERE ended_at IS NULL AND {}), 0)
        FROM time_entries
        WHERE {} AND ended_at IS NOT NULL
    "#,
        date_where, date_where
    ))
    .fetch_one(pool)
    .await?;

    // 活跃天数
    let active_days: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(DISTINCT date(started_at)) FROM time_entries WHERE {}",
        date_where
    ))
    .fetch_one(pool)
    .await?;

    // 计算天数
    let days_count: i64 = sqlx::query_scalar(&format!(
        "SELECT CAST((julianday({}) - julianday({}) + 1) AS INTEGER)",
        end_date, start_date
    ))
    .fetch_one(pool)
    .await?;
    let days_count = if days_count > 0 { days_count } else { 30 };

    // 日均
    let daily_average_seconds = if total_seconds > 0 {
        total_seconds / days_count
    } else {
        0
    };

    // 每日概览
    let month_data: Vec<MonthlyDailyStatSql> = sqlx::query_as::<_, MonthlyDailyStatSql>(&format!(
        r#"
        SELECT date(started_at) AS date,
               COALESCE(SUM(duration_seconds), 0) +
               COALESCE((SELECT strftime('%s', 'now') - strftime('%s', started_at)
                        FROM time_entries WHERE ended_at IS NULL AND date(started_at) = date(te.started_at)), 0) AS seconds
        FROM time_entries te
        WHERE {}
        GROUP BY date(started_at)
        ORDER BY date(started_at)
    "#,
        date_where
    ))
    .fetch_all(pool)
    .await?;

    // 转换为带 level 的格式
    let max_seconds = month_data.iter().map(|d| d.seconds).max().unwrap_or(0);

    // 生成范围内的每一天
    // 从 start_date 和 end_date 解析出日期范围
    let start_parsed = if let Some(ref s) = query.start_date {
        if s.len() == 7 {
            // YYYY-MM 格式，取该月第一天
            chrono::NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d")
                .unwrap_or_else(|_| chrono::Utc::now().date_naive())
        } else {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .unwrap_or_else(|_| chrono::Utc::now().date_naive())
        }
    } else {
        chrono::Utc::now().date_naive()
    };
    let end_parsed = if let Some(ref e) = query.end_date {
        chrono::NaiveDate::parse_from_str(e, "%Y-%m-%d")
            .unwrap_or_else(|_| start_parsed + chrono::Duration::days(29))
    } else if query.start_date.as_ref().map_or(false, |s| s.len() == 7) {
        // 如果只有 YYYY-MM，计算月末
        start_parsed + chrono::Duration::days(days_count - 1)
    } else {
        start_parsed + chrono::Duration::days(29)
    };

    let mut all_days: Vec<MonthlyDailyStat> = Vec::new();
    let mut current = start_parsed;
    while current <= end_parsed {
        let date = current.format("%Y-%m-%d").to_string();
        let day_data = month_data.iter().find(|d| d.date == date);
        let seconds = day_data.map(|d| d.seconds).unwrap_or(0);

        let active = seconds > 0;
        let level = if seconds == 0 {
            0
        } else if max_seconds > 0 {
            ((seconds as f64 / max_seconds as f64 * 3.0).ceil() as i32).clamp(1, 3)
        } else {
            1
        };

        all_days.push(MonthlyDailyStat {
            date,
            seconds,
            active,
            level,
        });
        current = current.succ_opt().unwrap();
    }

    // 分类统计
    let categories: Vec<CategoryStat> = sqlx::query_as::<_, CategoryStatSql>(&format!(
        r#"
        SELECT COALESCE(c.name, 'Other') AS name, COALESCE(c.color, '#9CA3AF') AS color,
               COALESCE(SUM(te.duration_seconds), 0) AS seconds
        FROM time_entries te
        LEFT JOIN tasks t ON te.task_id = t.id
        LEFT JOIN categories c ON t.category_id = c.id
        WHERE {}
        GROUP BY c.id
        ORDER BY seconds DESC
    "#,
        date_where
    ))
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|c| {
        let total = if total_seconds > 0 {
            total_seconds as f64
        } else {
            1.0
        };
        CategoryStat {
            name: c.name,
            color: c.color,
            seconds: c.seconds,
            percentage: (c.seconds as f64 / total * 100.0).round(),
        }
    })
    .collect();

    // Top 任务
    let top_tasks: Vec<TaskStat> = sqlx::query_as::<_, TaskStatSql>(&format!(
        r#"
        SELECT t.id AS task_id, t.title AS task_title, c.name AS category,
               c.color AS category_color,
               COALESCE(SUM(te.duration_seconds), 0) AS seconds,
               NULL AS last_time
        FROM time_entries te
        JOIN tasks t ON te.task_id = t.id
        LEFT JOIN categories c ON t.category_id = c.id
        WHERE {}
        GROUP BY t.id
        ORDER BY seconds DESC
        LIMIT 10
    "#,
        date_where
    ))
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|t| TaskStat {
        task_id: t.task_id,
        task_title: t.task_title,
        category: t.category,
        category_color: t.category_color,
        seconds: t.seconds,
        last_time: None,
    })
    .collect();

    Ok(MonthStatsResponse {
        total_seconds,
        daily_average_seconds,
        active_days,
        monthly_overview: all_days,
        categories,
        top_tasks,
    })
}

// SQL 查询辅助结构体
#[derive(Debug, FromRow)]
struct CategoryStatSql {
    name: String,
    color: String,
    seconds: i64,
}

#[derive(Debug, FromRow)]
struct DailyStatSql {
    day_num: String,
    date: String,
    seconds: i64,
}

#[derive(Debug, FromRow)]
struct MonthlyDailyStatSql {
    date: String,
    seconds: i64,
}

#[derive(Debug, FromRow)]
struct TaskStatSql {
    task_id: u32,
    task_title: String,
    category: Option<String>,
    category_color: Option<String>,
    seconds: i64,
    last_time: Option<String>,
}

fn format_time_only(datetime: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S") {
        dt.format("%-I:%M %p").to_string().to_lowercase()
    } else {
        datetime.to_string()
    }
}
