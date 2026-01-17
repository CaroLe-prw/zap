export function formatSmartDate(completedAt: string | null): string {
  if (!completedAt) return "";
  // 后端返回的是 UTC 时间，格式: 2026-01-17 04:54:45
  const date = new Date(completedAt.replace(" ", "T") + "Z");
  const now = new Date();

  const isToday = date.toDateString() === now.toDateString();
  const isYesterday =
    new Date(now.getTime() - 24 * 60 * 60 * 1000).toDateString() ===
    date.toDateString();
  const isThisYear = date.getFullYear() === now.getFullYear();

  if (isToday) {
    return "Today";
  }

  if (isYesterday) {
    return "Yesterday";
  }

  // 格式化日期部分
  const dateOptions: Intl.DateTimeFormatOptions = {
    month: "short",
    day: "numeric",
  };
  if (!isThisYear) {
    dateOptions.year = "numeric";
  }

  return date.toLocaleDateString("en-US", dateOptions);
}

export function formatCompletedTime(completedAt: string | null): string {
  if (!completedAt) return "";
  const date = new Date(completedAt.replace(" ", "T") + "Z");
  const timeStr = date.toLocaleTimeString("en-US", {
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });
  return `${formatSmartDate(completedAt)} ${timeStr}`;
}
