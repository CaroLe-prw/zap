// 根据颜色生成柔和的浅色背景和深色文字
export function getSoftColor(hexColor: string): { color: string; textColor: string } {
  // 将 hex 转为 RGB
  const r = parseInt(hexColor.slice(1, 3), 16);
  const g = parseInt(hexColor.slice(3, 5), 16);
  const b = parseInt(hexColor.slice(5, 7), 16);

  // 生成浅色背景 (添加白色，透明度约 30%)
  const bgR = Math.round(r + (255 - r) * 0.85);
  const bgG = Math.round(g + (255 - g) * 0.85);
  const bgB = Math.round(b + (255 - b) * 0.85);
  const bgColor = `rgb(${bgR}, ${bgG}, ${bgB})`;

  // 生成深色文字 (降低亮度)
  const textR = Math.round(r * 0.6);
  const textG = Math.round(g * 0.6);
  const textB = Math.round(b * 0.6);
  const textColor = `rgb(${textR}, ${textG}, ${textB})`;

  return { color: bgColor, textColor };
}
