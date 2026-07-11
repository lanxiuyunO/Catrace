# Toast 卡片紧凑尺寸规范 & 阴影防裁剪出血方案

2026-07-11 将 Toast 卡片整体缩小到接近 Windows 11 原生通知的紧凑度，并修复阴影四边被裁剪的问题。

## 尺寸规范（对标 Win11 原生 toast）

| 项 | 值 | 位置 |
|----|----|------|
| 窗口宽 | 360px | `WINDOW_WIDTH`（前端）/ `TOAST_WINDOW_WIDTH`（Rust） |
| 卡片宽 | 跟随窗口（= 窗口宽 − root padding × 2） | `.toast-card { width: 100% }` |
| 单条初始窗口高 | 160px = 卡片 128 + 上下 16 | Rust `TOAST_WINDOW_MIN_HEIGHT` |
| 卡片最小高 | 128px（`.toast-card min-height: 8rem`） | `CARD_HEIGHT` 常量同步 |
| 卡片 padding | 12px | `.toast-card padding: 0.75rem` |
| root 留白 | 16px（`PADDING = 16`） | 也是阴影出血空间 |
| 卡片间距 | 8px（`CARD_GAP = 8`） | `.toast-stack gap: 0.5rem` |
| 标题 | 14px / 700 | `.title font-size: 0.875rem` |
| 正文 | 13px / 行高 1.5 | `.body-text font-size: 0.8125rem` |
| 按钮 | 28px 高 / 12px 字 | `.btn height: 1.75rem / font-size: 0.75rem` |
| 计时球 | 84px | `.liquid-ball 5.25rem` |

**三处常量必须保持一致**：前端 `ReminderToast.vue` 的 `CARD_HEIGHT / CARD_GAP / PADDING / WINDOW_WIDTH`、CSS 里的 rem 值、Rust `reminder_toast.rs` 的 `TOAST_WINDOW_WIDTH / TOAST_WINDOW_MIN_HEIGHT`。改任何一个都要同步另外两处，否则窗口初始尺寸与前端实测高度对不上。

`EyeToastCard.vue` 是独立组件，尺寸规范同样适用（标题/按钮/间距与通用卡片一致）。

## 阴影为什么会被裁掉

症状：透明窗口里卡片只有底部有阴影，左右/顶部没有。

根因链：
1. 窗口是 `transparent(true)` 的无边框 WebviewWindow，超出窗口范围的像素直接不存在。
2. `.toast-stack` 设了 `overflow-y: auto`（卡片超出可滚动），而 CSS 里 overflow 任一轴非 visible 都会让另一轴也变成裁剪区。
3. 卡片 `width: 100%` 正好顶到 stack 边缘，左右/顶部阴影的出血空间为 0，被裁干净；底部因为 root 有 padding 才幸存。

## 修法：四边借出血 + 高度补偿

`.toast-stack` 四边各借 16px padding 放阴影，负 margin 拉回，卡片宽度不变：

```css
.toast-stack {
  overflow-y: auto;
  margin: -1rem;
  padding: 1rem;
}
```

副作用：stack 的 `scrollHeight` 因此多了 32px（上下 padding），直接拿去算窗口高度会让窗口变高、卡片离底边远。`adjustWindowSize()` 里要减掉：

```ts
const rawStackHeight = stackRef.value?.scrollHeight
const stackHeight = rawStackHeight != null ? rawStackHeight - 32 : calcWindowHeight(count)
```

**出血值和减去的 32px 是绑定的**：padding 改了，减的值要同步改成 padding × 2。

## 阴影本身

双层阴影，比单层更贴近原生观感：

```css
box-shadow:
  0 0.5rem 1.5rem rgba(0, 0, 0, 0.18),   /* 主阴影 */
  0 0.125rem 0.375rem rgba(0, 0, 0, 0.12); /* 贴边层 */
```

主阴影 blur 24px > root padding 16px，边缘会被窗口裁掉一小段，视觉上仍可接受；要继续加深阴影就同步加大 root padding（并更新 `PADDING` 与 Rust 高度常量）。

## 通用教训

透明无边框窗口里做卡片阴影：**任何带 overflow 的祖先都是裁剪区**，卡片贴边布局下阴影必然被裁。固定套路是「容器加 padding 借出血 + 负 margin 拉回 + 测量高度时减去 padding」。
