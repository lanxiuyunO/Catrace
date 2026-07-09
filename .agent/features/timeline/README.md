# 时间轴

Dashboard 下方活动可视化，支持概览和详细两种视图。

## 涉及文件

- `src/components/TimelineWindows.vue` — 概览视图
- `src/components/Timeline.vue` — 详细视图
- `src/utils/timeBlocks.ts` — `computeTimeBlocks` / `mergeRestBlocks`

## 概览视图（TimelineWindows，默认）

基于 block 切分的卡片网格 `repeat(auto-fit, minmax(15.625rem, 1fr))`。

每张卡片：时间范围 · 时长 · 状态标签。当前 block 紫边框 + 涟漪圆点 + "进行中"标签。

**时间范围**：已完成 block 结束用 `endTs + 60` 不含边界，对齐时长。时长 = 记录条数。

**展开视图**：点击卡片整行同步展开。每 10 分钟一行的混合分钟条：
- 连续 ≥5 分钟 → 连续色条（flex 比例宽度）
- <5 分钟 → 独立 8×8px 方块
- hover 上浮 + 高亮，自定义 tooltip
- 末行不足 10 分钟宽度按实际占比

## 详细视图（Timeline）

CSS Grid（24 行 × 60 列），每格 1 分钟 `<div>` 色块。鼠标移动显示时间和状态。当前分钟红色脉冲。

色值：活跃 `#7C3AED` / 休息 `#059669` / 无记录灰。
