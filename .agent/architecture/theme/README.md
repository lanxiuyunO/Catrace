# UI 主题

## 色板

| 用途 | 色值 |
|------|------|
| 页面背景 | `#F7F5FA` |
| 卡片 / 侧边栏 | `#FFFFFF` |
| 边框 | `#EBE6F2` |
| 主色（活跃） | `#7C3AED` / `#6D28D9` |
| 辅色（休息） | `#059669` |
| 标题文字 | `#2E1065` |
| 次要文字 | `#8B7AAB` |

## 实现

- `src/theme.ts` 定义色板常量 + `naive-ui` 的 `GlobalThemeOverrides`
- `App.vue` 通过 `NConfigProvider :theme-overrides` 注入
- 设计原则：克制干净——白卡片+细边框+轻阴影，颜色仅用于圆点/数值/标签
- 改主题样式时优先改 `theme.ts`
